use super::components::*;
use super::{Map, Position, RandomNumberGenerator};
use bracket_lib::prelude::*;
use specs::prelude::*;

/// System that controls monster AI
pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Position>, // Player position
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Vision>,
    );

    fn run(&mut self, (map, player_pos, monster, name, mut pos, mut vision): Self::SystemData) {
        let mut rng = RandomNumberGenerator::new();
        for (_monster, name, pos, vision) in (&monster, &name, &mut pos, &mut vision).join() {
            // If the player is visible, chase it or attack if in range
            if vision.visible.contains(&*player_pos) {
                let distance = DistanceAlg::Pythagoras.distance2d(
                    Point::new(pos.x, pos.y),
                    Point::new(player_pos.x, player_pos.y),
                );
                if distance < 1.5 {
                    // TODO: Attack!
                    console::log(format!("{} spits at you.", name.name));
                } else {
                    // Chase player
                    let path = a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &*map,
                    );
                    if path.success && path.steps.len() > 1 {
                        // TODO: encapsulate actual movement in another function
                        *pos = Position::from(map.idx_xy(path.steps[1]));
                        vision.recompute = true;
                    }
                }
            } else {
                // If the player is not visible, pick a random available direction and move in that direction
                let pos_idx = map.xy_idx(pos.x, pos.y);
                let available_exits = map.get_available_exits(pos_idx);
                let new_pos = rng.random_slice_entry(&available_exits);
                if let Some(new_pos) = new_pos {
                    let (x, y) = map.idx_xy(new_pos.0);
                    *pos = Position::new(x, y);
                    vision.recompute = true;
                }
            }
        }
    }
}
