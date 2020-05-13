use tcod::{FontLayout, FontType};
use tcod::console;
use tcod::map::Map as FovMap;

use crate::config::{MAP_HEIGHT, MAP_WIDTH};

pub struct Tcod {
    pub root: console::Root,
    pub con: console::Offscreen,
    pub fov: FovMap,
}

impl Tcod {
    pub fn new() -> Self {
        let root = console::Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(MAP_WIDTH, MAP_WIDTH)
            .title("Rust/libtcod tutorial")
            .init();

        Tcod {
            root,
            con: console::Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
            fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        }
    }
}



