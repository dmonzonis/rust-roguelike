use bracket_lib::prelude::*;
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
mod drunkard_system;
mod map;
mod render;
mod room;
mod visibility_system;

use crate::components::*;
use crate::drunkard_system::DrunkardSystem;
use crate::map::{Map, TileType};
use crate::render::{draw_entities, draw_map};
use crate::visibility_system::VisibilitySystem;

const CONSOLE_WIDTH: i32 = 80;
const CONSOLE_HEIGHT: i32 = 50;
const TILE_SIZE: i32 = 16;

// Other functions

/// Try moving players by the given diff and return whether the move was successful or not
fn try_move_player(dx: i32, dy: i32, ecs: &mut World) -> bool {
    let mut success = false;
    let map = ecs.fetch::<Map>();
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    for (pos, _player) in (&mut positions, &players).join() {
        let new_pos = Position {
            x: pos.x + dx,
            y: pos.y + dy,
        };
        let new_pos_idx = map.xy_idx(new_pos.x, new_pos.y);
        if map.tiles[new_pos_idx] != TileType::Wall
            && map.in_bounds(Point::new(new_pos.x, new_pos.y))
        {
            *pos = new_pos;
            success = true;
        }
    }

    // If the player moved, we need to recompute FOV
    if success {
        let mut visions = ecs.write_storage::<Vision>();
        for (vision, _player) in (&mut visions, &players).join() {
            vision.recompute = true;
        }
    }
    success
}

/// Take player input and return whether a turn was taken or not
fn player_input(ecs: &mut World, ctx: &mut BTerm) -> bool {
    // Player movement
    match ctx.key {
        None => false, // No key is being pressed
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, ecs),
            _ => false,
        },
    }
}

// Main game state

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        VisibilitySystem {}.run_now(&self.ecs);
        DrunkardSystem {}.run_now(&self.ecs);
        // Apply now all changes to the ECS that may be queued from running the systems
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let turn_taken = player_input(&mut self.ecs, ctx);

        // Run systems
        if turn_taken {
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
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Drunkard>();
    gs.ecs.register::<Vision>();

    // Add map resource
    let map = Map::new(CONSOLE_WIDTH, CONSOLE_HEIGHT);
    let player_pos = map.rooms[0].center();
    gs.ecs.insert(map);

    // Create player entity
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_pos.0,
            y: player_pos.1,
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

    // Create some drunkards in rooms other than the first one (where the player spawns)
    let rooms;
    {
        let map = gs.ecs.fetch::<Map>();
        rooms = map.rooms.clone();
    }
    for room in rooms.iter().skip(1) {
        let pos = room.center();
        gs.ecs
            .create_entity()
            .with(Position { x: pos.0, y: pos.1 })
            .with(Renderable {
                glyph: to_cp437('D'),
                fg: RGB::named(DARK_GREEN),
                bg: RGB::named(BLACK),
            })
            .with(Drunkard {})
            .build();
    }

    // Run some systems that need to be run before the first turn
    VisibilitySystem {}.run_now(&gs.ecs);

    main_loop(context, gs);
}
