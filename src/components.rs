use bracket_lib::prelude::*;
use specs::prelude::*;
use std::convert::TryInto;
use std::ops;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: TryInto<i32>,
    {
        Self {
            x: x.try_into().ok().unwrap(),
            y: y.try_into().ok().unwrap(),
        }
    }
}

impl<T: TryInto<i32>> From<(T, T)> for Position {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0.try_into().ok().unwrap(),
            y: tuple.1.try_into().ok().unwrap(),
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Multiplication of a position by a scalar (integer)
impl ops::Mul<i32> for Position {
    type Output = Self;
    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Dot product
impl ops::Mul<Position> for Position {
    type Output = i32;
    fn mul(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }
}

/// Name of the entity
#[derive(Component)]
pub struct Name {
    pub name: String,
}

/// Entities that should be rendered to the screen
#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

/// Component that identifies the player character
#[derive(Component)]
pub struct Player {}

/// Component for entities that can see things following a FOV algorithm.
#[derive(Component)]
pub struct Vision {
    pub visible: Vec<Position>,
    pub range: i32,
    pub recompute: bool,
}

/// Is controlled by monster AI (MonsterAISystem), that chases and attacks the player.
#[derive(Component)]
pub struct Monster {}

/// Whether this entity blocks other entities from occupying the same tile.
#[derive(Component)]
pub struct Blocking {}

/// Basic struct for entities that can fight and die
#[derive(Component)]
pub struct Fighter {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
}
