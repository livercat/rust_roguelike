use tcod::{FontLayout, FontType};
use tcod::console;

use crate::config;

pub struct Tcod {
    pub root: console::Root,
    pub con: console::Offscreen,
}

impl Tcod {
    pub fn new() -> Self {
        let root = console::Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(config::MAP_WIDTH, config::MAP_WIDTH)
            .title("Rust/libtcod tutorial")
            .init();
        let con = console::Offscreen::new(config::MAP_WIDTH, config::MAP_HEIGHT);

        Tcod { root, con }
    }
}



