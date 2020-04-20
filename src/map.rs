use std::cmp;
use std::ops::Range;

use rand::Rng;
use tcod::Color;

use crate::actors;
use crate::config::{MAP_HEIGHT, MAP_WIDTH, MAX_ROOMS, ROOM_MAX_SIZE, ROOM_MIN_SIZE};

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

#[derive(Clone, Debug)]
pub struct Map {
    map: Vec<Vec<Tile>>,
    rooms: Vec<Rect>,
}

/// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub impassable: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            impassable: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            impassable: true,
            block_sight: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

fn make_range(v1: i32, v2: i32) -> Range<i32> {
    cmp::min(v1, v2)..(cmp::max(v1, v2) + 1)
}

impl Map {
    fn create_room(&mut self, room: Rect) {
        // go through the tiles in the rectangle and make them passable
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.map[x as usize][y as usize] = Tile::empty();
            }
        }
    }

    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
        for x in make_range(x1, x2) {
            self.map[x as usize][y as usize] = Tile::empty();
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        // vertical tunnel
        for y in make_range(y1, y2) {
            self.map[x as usize][y as usize] = Tile::empty();
        }
    }

    fn connect_new_room_with_previous(&mut self, new_room: &Rect) {
        let (new_x, new_y) = new_room.center();
        // center coordinates of the previous room
        let (prev_x, prev_y) = self.rooms[self.rooms.len() - 2].center();

        // toss a coin (random bool value -- either true or false)
        if rand::random() {
            // first move horizontally, then vertically
            self.create_h_tunnel(prev_x, new_x, prev_y);
            self.create_v_tunnel(prev_y, new_y, new_x);
        } else {
            // first move vertically, then horizontally
            self.create_v_tunnel(prev_y, new_y, prev_x);
            self.create_h_tunnel(prev_x, new_x, new_y);
        }
    }

    pub fn is_passable(&self, x: i32, y: i32) -> bool {
        !self.map[x as usize][y as usize].impassable
    }

    pub fn blocks_sight(&self, x: i32, y: i32) -> bool {
        self.map[x as usize][y as usize].block_sight
    }

    pub fn new(npc_vec: &mut Vec<actors::Actor>) -> Map {
        let mut map = Map {
            map: vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            rooms: vec![],
        };
        map.fill(npc_vec);
        map
    }

    pub fn fill(&mut self, npc_vec: &mut Vec<actors::Actor>) {
        let mut npc_iter = npc_vec.iter_mut();

        for _ in 0..MAX_ROOMS {
            // random width and height
            let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
            let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

            let new_room = Rect::new(x, y, w, h);

            // run through the other rooms and see if they intersect with this one
            let failed = self.rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));
            if failed {
                continue;
            }
            // this means there are no intersections, so this room is valid

            // "paint" it to the map's tiles
            self.create_room(new_room);
            let (new_x, new_y) = new_room.center();
            if let Some(next_npc) = npc_iter.next() {
                next_npc.set_coords(new_x, new_y);
            }

            self.rooms.push(new_room);
            if self.rooms.len() == 1 {
                continue;
            }
            // all rooms after the first:
            // connect it to the previous room with a tunnel
            self.connect_new_room_with_previous(&new_room);
        }
    }
}
