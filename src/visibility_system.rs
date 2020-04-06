use super::{Map, Position, Vision};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Vision>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut map, mut vision, pos): Self::SystemData) {
        for (vision, pos) in (&mut vision, &pos).join() {
            vision.visible.clear();
            vision.visible = field_of_view(Point::new(pos.x, pos.y), vision.range, &*map);
            vision
                .visible
                .retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1);
            // Mark viewed tiles as explored
            // TODO: Improve performance
            for point in &vision.visible {
                let idx = map.xy_idx(point.x, point.y);
                map.explored[idx] = true;
            }
        }
    }
}
