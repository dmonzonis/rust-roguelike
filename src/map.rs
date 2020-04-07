use bracket_lib::prelude::*;

use super::room::Room;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Room>,
    pub width: i32,
    pub height: i32,
    pub explored: Vec<bool>,
    pub visible: Vec<bool>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let total_size = (width * height) as usize;
        let mut map = Self {
            tiles: vec![TileType::Wall; total_size],
            rooms: Vec::new(),
            width,
            height,
            explored: vec![false; total_size],
            visible: vec![false; total_size]
        };

        let mut rooms: Vec<Room> = Vec::new();
        const MAX_ROOMS: i32 = 12;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 12;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let width = rng.range(MIN_SIZE, MAX_SIZE);
            let height = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(1, map.width - width - 2);
            let y = rng.range(1, map.height - height - 2);
            let room = Room::new(x, y, width, height);
            let mut valid = true;
            for other_room in rooms.iter() {
                if room.intersects(other_room) {
                    valid = false;
                    break;
                }
            }
            if valid {
                map.carve_room(&room);
                // Connect to the previous room
                if !rooms.is_empty() {
                    let (new_x, new_y) = room.center();
                    let (prev_x, prev_y) = rooms.last().unwrap().center();
                    if rng.range(0, 1) == 1 {
                        map.carve_corridor_horizontal(prev_x, new_x, prev_y);
                        map.carve_corridor_vertical(prev_y, new_y, new_x);
                    } else {
                        map.carve_corridor_vertical(prev_y, new_y, prev_x);
                        map.carve_corridor_horizontal(prev_x, new_x, new_y);
                    }
                }

                rooms.push(room);
            }
        }

        map.rooms = rooms;
        map
    }

    pub fn carve_room(&mut self, room: &Room) {
        for y in room.y0..=room.y1 {
            for x in room.x0..=room.x1 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    // Carves a corridor from x0 to x1 (inclusive) at row y
    pub fn carve_corridor_horizontal(&mut self, x0: i32, x1: i32, y: i32) {
        for x in min(x0, x1)..=max(x0, x1) {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    // Carves a corridor from y0 to y1 (inclusive) at column x
    pub fn carve_corridor_vertical(&mut self, y0: i32, y1: i32, x: i32) {
        for y in min(y0, y1)..=max(y0, y1) {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
        }
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

// Implement traits for FOV and pathfinding algorithms to work
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}
