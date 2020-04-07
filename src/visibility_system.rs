use super::{Map, Player, Position, Vision};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Vision>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut map, entities, mut vision, pos, player): Self::SystemData) {
        for (ent, vision, pos) in (&entities, &mut vision, &pos).join() {
            if vision.recompute {
                vision.recompute = false;
                vision.visible.clear();
                let mut visible_pts: Vec<Point> =
                    field_of_view(Point::new(pos.x, pos.y), vision.range, &*map);
                // Remove out of bounds tiles. For this, use the in_bounds method we get for free from
                // implementing BaseMap
                visible_pts.retain(|p| map.in_bounds(Point::new(p.x, p.y)));
                vision.visible = visible_pts
                    .iter()
                    .map(|p| Position::new(p.x, p.y))
                    .collect();

                // Mark viewed tiles as explored if this is the player, and set visible tiles
                // TODO: Improve performance
                if let Some(_) = player.get(ent) {
                    for t in map.visible.iter_mut() {
                        *t = false;
                    }
                    for pos in &vision.visible {
                        let idx = map.xy_idx(pos.x, pos.y);
                        map.explored[idx] = true;
                        map.visible[idx] = true;
                    }
                }
            }
        }
    }
}
