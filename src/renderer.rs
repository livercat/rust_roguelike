use tcod::{BackgroundFlag, FontLayout, FontType, Console};
use tcod::console;
use tcod::map::Map as FovMap;

use crate::{actors, map};
use crate::config::{MAP_HEIGHT, MAP_WIDTH,
                    TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO,
                    COLOR_DARK_WALL, COLOR_DARK_GROUND, COLOR_LIGHT_WALL, COLOR_LIGHT_GROUND};

pub struct Tcod {
    pub root: console::Root,
    con: console::Offscreen,
    fov: FovMap,
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

    pub fn init_fov(&mut self, map: &map::Map) -> () {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                self.fov.set(
                    x,
                    y,
                    !map.blocks_sight(x, y),
                    !map.is_passable(x, y),
                );
            }
        }
    }

    /// set the color and then draw the character that represents this object at its position
    fn draw_actor(&mut self, actor: &actors::Actor) {
        if self.fov.is_in_fov(actor.x(), actor.y()) {
            self.con.set_default_foreground(actor.color());
            self.con.put_char(actor.x(), actor.y(), actor.char(), BackgroundFlag::None);
        }
    }

    pub fn render_all(&mut self, map: &mut map::Map, actors: &actors::Actors, recompute_fov: bool) {
        // clear the screen of the previous frame
        self.con.clear();

        if recompute_fov {
            // recompute FOV if needed (the player moved or something)
            self.fov.compute_fov(actors.player.x(),
                                      actors.player.y(),
                                      TORCH_RADIUS,
                                      FOV_LIGHT_WALLS,
                                      FOV_ALGO);
        }

        // draw all objects in the list
        self.draw_actor(&actors.player);
        for actor in &actors.npcs {
            self.draw_actor(actor);
        }

        // go through all tiles, and set their background color
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let visible = self.fov.is_in_fov(x, y);
                let wall = map.blocks_sight(x, y);
                if visible {
                    map.explore(x, y);
                }
                let color = match (visible, wall) {
                    // outside of field of view:
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,
                    // inside fov:
                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
                };
                if map.explored(x, y) {
                    self.con.set_char_background(x, y, color, BackgroundFlag::Set);
                }
            }
        }

        // blit the contents of "con" to the root console and present it
        console::blit(
            &self.con,
            (0, 0),
            (MAP_WIDTH, MAP_HEIGHT),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );

        self.root.flush();
    }
}



