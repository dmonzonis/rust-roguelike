use super::components::{Position, Renderable};
use super::map::{Map, TileType};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    for (idx, tile) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_xy(idx);
        if map.explored[idx] {
            let mut fg_color = RGB::named(DARK_BLUE);
            if map.visible[idx] {
                fg_color = RGB::named(WHITE);
            }
            match tile {
                TileType::Floor => {
                    ctx.set(x, y, fg_color, RGB::named(BLACK), to_cp437('.'));
                }
                TileType::Wall => {
                    ctx.set(x, y, fg_color, RGB::named(BLACK), to_cp437('#'));
                }
            }
        }
    }
}

pub fn draw_entities(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();

    for (pos, renderable) in (&positions, &renderables).join() {
        let pos_idx = map.xy_idx(pos.x, pos.y);
        if map.visible[pos_idx] {
            ctx.set(pos.x, pos.y, renderable.fg, renderable.bg, renderable.glyph);
        }
    }
}
