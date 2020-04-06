use bracket_lib::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Self {
            tiles: vec![TileType::Floor; (width * height) as usize],
            width,
            height,
        };

        // Make boundaries
        for x in 0..width {
            let mut idx = map.xy_idx(x, 0);
            map.tiles[idx] = TileType::Wall;
            idx = map.xy_idx(x, height - 1);
            map.tiles[idx] = TileType::Wall;
        }
        for y in 0..height {
            let mut idx = map.xy_idx(0, y);
            map.tiles[idx] = TileType::Wall;
            idx = map.xy_idx(width - 1, y);
            map.tiles[idx] = TileType::Wall;
        }

        // Randomly add some wall tiles
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..200 {
            let x = rng.roll_dice(1, width - 1);
            let y = rng.roll_dice(1, height - 1);
            let idx = map.xy_idx(x, y);
            if idx != map.xy_idx(40, 25) {
                // We are placing the player here
                map.tiles[idx] = TileType::Wall;
            }
        }
        map
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }
}
