use tcod::{Color};

use crate::map;

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Copy, Clone, Debug)]
pub struct Actor {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

pub struct Actors {
    pub player: Actor,
    pub npcs: Vec<Actor>,
}

impl Actor {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Actor { x, y, char, color }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn char(&self) -> char {
        self.char
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_coords(&mut self, x: i32, y: i32) -> () {
        self.x = x;
        self.y = y;
    }

    /// move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32, map: &map::Map) {
        if map.is_passable(self.x + dx, self.y + dy) {
            self.x += dx;
            self.y += dy;
        }
    }
}