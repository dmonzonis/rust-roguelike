use crate::{Blocking, Map, Position};
use specs::prelude::*;

pub struct MapManagementSystem {}

impl<'a> System<'a> for MapManagementSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Blocking>,
        Entities<'a>,
    );

    fn run(&mut self, (mut map, pos, block, entities): Self::SystemData) {
        map.compute_blocked();
        map.clear_entities();

        // TODO: Also update blocked tiles when any blocking entity moves, not just once per tick, as
        // many entities move in a tick and use this data
        for (pos, ent) in (&pos, &entities).join() {
            let idx = map.xy_idx(pos.x, pos.y);

            // If the entity has Blocking, update the map
            if let Some(_) = block.get(ent) {
                map.blocked[idx] = true;
            }

            map.tile_entities[idx].push(ent);
        }
    }
}
