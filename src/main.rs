use bracket_lib::prelude::*;
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
mod map;
mod map_management_system;
mod monster_ai;
mod player;
mod render;
mod room;
mod visibility_system;

use crate::components::*;
use crate::map::Map;
use crate::map_management_system::MapManagementSystem;
use crate::monster_ai::MonsterAISystem;
use crate::player::player_input;
use crate::render::{draw_entities, draw_map};
use crate::visibility_system::VisibilitySystem;

const CONSOLE_WIDTH: i32 = 80;
const CONSOLE_HEIGHT: i32 = 50;
const TILE_SIZE: i32 = 16;

// Main game state

#[derive(PartialEq, Copy, Clone)]
pub enum TurnState {
    Paused,
    Running,
}

pub struct State {
    ecs: World,
}

impl State {
    pub fn run_systems(&mut self) {
        VisibilitySystem {}.run_now(&self.ecs);
        MonsterAISystem {}.run_now(&self.ecs);
        MapManagementSystem {}.run_now(&self.ecs);
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
    gs.ecs.register::<Blocking>();
    gs.ecs.register::<Fighter>();

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
        .with(Fighter {
            max_hp: 30,
            hp: 30,
            attack: 5,
            defense: 2,
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
            .with(Blocking {})
            .with(Fighter {
                max_hp: 12,
                hp: 12,
                attack: 4,
                defense: 1,
            })
            .build();
    }

    // Run some systems that need to be run before the first turn
    VisibilitySystem {}.run_now(&gs.ecs);

    main_loop(context, gs);
}
