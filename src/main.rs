use bracket_lib::prelude::*;
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
mod map;
mod monster_ai;
mod render;
mod room;
mod visibility_system;

use crate::components::*;
use crate::map::{Map, TileType};
use crate::monster_ai::MonsterAISystem;
use crate::render::{draw_entities, draw_map};
use crate::visibility_system::VisibilitySystem;

const CONSOLE_WIDTH: i32 = 80;
const CONSOLE_HEIGHT: i32 = 50;
const TILE_SIZE: i32 = 16;

// Other functions

/// Try moving players by the given diff and return the new turn state (Running if the
/// player turn was successful or Paused if no turn was taken)
fn try_move_player(dx: i32, dy: i32, ecs: &mut World) -> TurnState {
    let mut success = false;
    let map = ecs.fetch::<Map>();
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    let mut player_pos_res = ecs.write_resource::<Position>();
    for (pos, _player) in (&mut positions, &players).join() {
        let new_pos = *pos + Position::new(dx, dy);
        let new_pos_idx = map.xy_idx(new_pos.x, new_pos.y);
        if map.in_bounds(Point::new(new_pos.x, new_pos.y))
            && map.tiles[new_pos_idx] != TileType::Wall
        {
            *pos = new_pos;
            // Also update the resource
            *player_pos_res = new_pos;
            success = true;
        }
    }

    // If the player moved, we need to recompute FOV
    if success {
        let mut visions = ecs.write_storage::<Vision>();
        for (vision, _player) in (&mut visions, &players).join() {
            vision.recompute = true;
        }
        return TurnState::Running;
    }
    TurnState::Paused
}

/// Take player input and return the new turn state
fn player_input(ecs: &mut World, ctx: &mut BTerm) -> TurnState {
    // Player movement
    match ctx.key {
        None => TurnState::Paused, // No key is being pressed
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, ecs)
            }
            _ => TurnState::Paused,
        },
    }
}

// Main game state

#[derive(PartialEq, Copy, Clone)]
enum TurnState {
    Paused,
    Running,
}

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        VisibilitySystem {}.run_now(&self.ecs);
        MonsterAISystem {}.run_now(&self.ecs);
        // Apply now all changes to the ECS that may be queued from running the systems
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let turn_state = player_input(&mut self.ecs, ctx);

        // Run systems
        if turn_state == TurnState::Running {
            self.run_systems();
        }

        // Render stuff
        draw_map(&self.ecs, ctx);
        draw_entities(&self.ecs, ctx);
    }
}

embedded_resource!(TILE_FONT, "../res/terminal16x16.png");

fn main() {
    link_resource!(TILE_FONT, "resources/terminal16x16.png");

    let context = BTermBuilder::new()
        .with_dimensions(CONSOLE_WIDTH, CONSOLE_HEIGHT)
        .with_tile_dimensions(TILE_SIZE, TILE_SIZE)
        .with_title("Roguelike Test")
        .with_font("terminal16x16.png", TILE_SIZE, TILE_SIZE)
        .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, "terminal16x16.png")
        .build();
    let mut gs = State { ecs: World::new() };

    // ECS setup
    // Register all components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Vision>();
    gs.ecs.register::<Monster>();

    // Add ECS resources: map, and player position
    let map = Map::new(CONSOLE_WIDTH, CONSOLE_HEIGHT);
    let player_pos = Position::from(map.rooms[0].center());
    gs.ecs.insert(map);
    gs.ecs.insert(player_pos);

    // Create player entity
    gs.ecs
        .create_entity()
        .with(player_pos)
        .with(Name {
            name: String::from("Hero"),
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(Vision {
            visible: Vec::new(),
            range: 8,
            recompute: true,
        })
        .build();

    // Create some monsters in rooms other than the first one (where the player spawns)
    let rooms;
    {
        let map = gs.ecs.fetch::<Map>();
        rooms = map.rooms.clone();
    }
    for room in rooms.iter().skip(1) {
        let pos = Position::from(room.center());
        gs.ecs
            .create_entity()
            .with(pos)
            .with(Name {
                name: String::from("Orc"),
            })
            .with(Renderable {
                glyph: to_cp437('o'),
                fg: RGB::named(GREEN),
                bg: RGB::named(BLACK),
            })
            .with(Vision {
                visible: Vec::new(),
                range: 8,
                recompute: true,
            })
            .with(Monster {})
            .build();
    }

    // Run some systems that need to be run before the first turn
    VisibilitySystem {}.run_now(&gs.ecs);

    main_loop(context, gs);
}
