use super::{Map, Position, RandomNumberGenerator, TileType};
use super::components::Drunkard;
use bracket_lib::prelude::*;
use specs::prelude::*;

/// System that makes entities with Drunkard component move randomly each turn
pub struct DrunkardSystem {}

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
            if map.tiles[new_pos_idx] != TileType::Wall
                && map.in_bounds(Point::new(new_pos.x, new_pos.y))
            {
                *pos = new_pos;
            }
        }
    }
}
