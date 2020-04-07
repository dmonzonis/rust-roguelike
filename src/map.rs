use super::room::Room;
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        // To complete as more tile types are added
        // TODO: Move this data to a JSON config file and load from there
        match self {
            TileType::Wall => false,
            TileType::Floor => true,
        }
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Room>,
    pub width: i32,
    pub height: i32,
    pub explored: Vec<bool>,
    pub visible: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_entities: Vec<Vec<Entity>>,
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
            visible: vec![false; total_size],
            blocked: vec![false; total_size],
            tile_entities: vec![Vec::new(); total_size],
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

    /// Carves a corridor from x0 to x1 (inclusive) at row y
    pub fn carve_corridor_horizontal(&mut self, x0: i32, x1: i32, y: i32) {
        for x in min(x0, x1)..=max(x0, x1) {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    /// Carves a corridor from y0 to y1 (inclusive) at column x
    pub fn carve_corridor_vertical(&mut self, y0: i32, y1: i32, x: i32) {
        for y in min(y0, y1)..=max(y0, y1) {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    /// Computes the blocked array using the tile composition. Doesn't take into account blocking entities.
    pub fn compute_blocked(&mut self) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            self.blocked[idx] = !tile.is_walkable();
        }
    }

    pub fn clear_entities(&mut self) {
        for entities in self.tile_entities.iter_mut() {
            entities.clear();
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

// Implement traits for FOV and pathfinding algorithms to work properly
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    // From a position given by idx, return the positions an entity can move to, and the cost to move
    // to that tile from the origin
    fn get_available_exits(&self, origin: usize) -> Vec<(usize, f32)> {
        let origin = self.index_to_point2d(origin);
        let mut exits: Vec<(usize, f32)> = Vec::new();
        let orthogonal = [
            Point { x: 1, y: 0 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
        ];
        let diagonal = [
            Point { x: 1, y: 1 },
            Point { x: -1, y: -1 },
            Point { x: -1, y: 1 },
            Point { x: 1, y: -1 },
        ];
        // Orthogonal movement costs 1
        for dir in orthogonal.iter() {
            let new_pos = origin + *dir;
            let new_pos_idx = self.point2d_to_index(new_pos);
            if self.in_bounds(new_pos) && !self.blocked[new_pos_idx] {
                // For now, all tiles have cost 1
                exits.push((new_pos_idx, 1.0));
            }
        }
        // Diagonal movement costs ~sqrt(2)
        for dir in diagonal.iter() {
            let new_pos = origin + *dir;
            let new_pos_idx = self.point2d_to_index(new_pos);
            if self.in_bounds(new_pos) && !self.blocked[new_pos_idx] {
                // For now, all tiles have cost 1
                exits.push((new_pos_idx, 1.4142));
            }
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let point1 = self.index_to_point2d(idx1);
        let point2 = self.index_to_point2d(idx2);
        DistanceAlg::Pythagoras.distance2d(point1, point2)
    }
}
