use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

#[macro_use]
extern crate specs_derive;

mod components;
mod map;
use crate::components::*;
use crate::map::*;

// Systems

/// System that makes entities with Drunkard component move randomly each turn
struct DrunkardSystem {}

impl<'a> System<'a> for DrunkardSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Drunkard>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (map, drunk, mut pos): Self::SystemData) {
        let mut rng = RandomNumberGenerator::new();
        let directions = [
            Position { x: 1, y: 0 },
            Position { x: -1, y: 0 },
            Position { x: 0, y: 1 },
            Position { x: 0, y: -1 },
        ];
        for (_drunk, pos) in (&drunk, &mut pos).join() {
            // Pick a random direction and move in that direction if possible
            let dir = rng.random_slice_entry(&directions).unwrap();
            // TODO: This next part is pretty similar to try_move_player; refactor to avoid duplicate code
            let new_pos = Position {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
            };
            let new_pos_idx = map.xy_idx(new_pos.x, new_pos.y);
            if map.tiles[new_pos_idx] != TileType::Wall {
                *pos = new_pos;
            }
        }
    }
}

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
        if map.tiles[new_pos_idx] != TileType::Wall {
            *pos = new_pos;
            success = true;
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

fn draw_map(map: &Map, ctx: &mut BTerm) {
    for (idx, tile) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_xy(idx);
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('#'));
            }
        }
    }
}

// Main game state

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut drunkard_system = DrunkardSystem {};
        drunkard_system.run_now(&self.ecs);
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
        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, renderable) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, renderable.fg, renderable.bg, renderable.glyph);
        }
    }
}

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Test")
        .build();
    let mut gs = State { ecs: World::new() };

    // ECS setup
    // Register all components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Drunkard>();

    // Add map resource
    gs.ecs.insert(Map::new(80, 50));

    // Create player entity
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .build();

    // Create some drunkards
    let mut rng = RandomNumberGenerator::new();
    let width;
    let height;
    {
        let map = gs.ecs.fetch::<Map>();
        width = map.width;
        height = map.height;
    }
    for _i in 0..4 {
        let x = rng.range(0, width - 1);
        let y = rng.range(0, height - 1);
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: to_cp437('D'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(Drunkard {})
            .build();
    }

    main_loop(context, gs);
}