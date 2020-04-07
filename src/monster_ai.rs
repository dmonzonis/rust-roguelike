use super::components::*;
use super::{Map, Position, RandomNumberGenerator};
use bracket_lib::prelude::*;
use specs::prelude::*;

/// System that controls monster AI
pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Vision>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (map, monster, mut pos, vision, player): Self::SystemData) {
        let mut rng = RandomNumberGenerator::new();
        for (_monster, pos) in (&monster, &mut pos).join() {
            // Pick a random available direction and move in that direction
            let pos_idx = map.xy_idx(pos.x, pos.y);
            let available_exits = map.get_available_exits(pos_idx);
            let new_pos = rng.random_slice_entry(&available_exits);
            if let Some(new_pos) = new_pos {
                let (x, y) = map.idx_xy(new_pos.0);
                *pos = Position { x, y };
            }
        }
    }
}
