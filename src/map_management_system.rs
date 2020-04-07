use specs::prelude::*;
use crate::{Map, Position, Blocking};

pub struct MapManagementSystem {}

/// Update the map with info about blocking entities
impl<'a> System<'a> for MapManagementSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Blocking>
    );

    fn run(&mut self, (mut map, pos, block): Self::SystemData) {
        map.compute_blocked();
        for (pos, _block) in (&pos, &block).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            map.blocked[idx] = true;
        }
    }
}
