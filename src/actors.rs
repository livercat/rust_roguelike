use tcod::{BackgroundFlag, Color, Console};

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

impl Actor {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Actor { x, y, char, color }
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

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}