/// For now a room is just a rectangle
pub struct Room {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x0: x,
            y0: y,
            x1: x + width,
            y1: y + height,
        }
    }

    /// Return true if the room intersects with another room
    pub fn intersects(&self, other: &Room) -> bool {
        self.x0 <= other.x1 && self.x1 >= other.x0 && self.y0 <= other.y1 && self.y1 >= other.y0
    }

    /// Returns the center (or an approximation) of the room
    pub fn center(&self) -> (i32, i32) {
        let x = (self.x0 + self.x1) / 2;
        let y = (self.y0 + self.y1) / 2;
        (x, y)
    }
}
