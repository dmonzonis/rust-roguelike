use bracket_lib::prelude::*;
use specs::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// TODO: Implement some operations (addition, product by scalar, etc)

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Drunkard {}

/// Component for entities that can see things following a FOV algorithm
#[derive(Component)]
pub struct Vision {
    pub visible: Vec<Point>,
    pub range: i32,
    pub recompute: bool,
}
