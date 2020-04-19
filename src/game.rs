use tcod::{BackgroundFlag, Console, console, system};
use tcod::input::{Key, KeyCode};

use crate::{actors, config, map, screen};

pub struct Game {
    map: map::Map,
    actors: Vec<actors::Actor>,
    tcod: screen::Tcod,
}

impl Game {
    pub fn new(tcod: screen::Tcod, map: map::Map, actors: Vec<actors::Actor>) -> Self {
        Game { map, actors, tcod }
    }

    fn render_all(&mut self) {
        // clear the screen of the previous frame
        self.tcod.con.clear();

        // draw all objects in the list
        for actor in &self.actors {
            actor.draw(&mut self.tcod.con);
        }

        // go through all tiles, and set their background color
        for y in 0..config::MAP_HEIGHT {
            for x in 0..config::MAP_WIDTH {
                let wall = self.map[x as usize][y as usize].block_sight;
                if wall {
                    self.tcod.con
                        .set_char_background(x, y, map::COLOR_DARK_WALL, BackgroundFlag::Set);
                } else {
                    self.tcod.con
                        .set_char_background(x, y, map::COLOR_DARK_GROUND, BackgroundFlag::Set);
                }
            }
        }

        // blit the contents of "con" to the root console and present it
        console::blit(
            &self.tcod.con,
            (0, 0),
            (config::MAP_WIDTH, config::MAP_HEIGHT),
            &mut self.tcod.root,
            (0, 0),
            1.0,
            1.0,
        );

        self.tcod.root.flush();
    }

    fn handle_keys(tcod: &mut screen::Tcod, map: &map::Map, player: &mut actors::Actor) -> bool {
        let key = tcod.root.wait_for_keypress(true);
        match key {
            Key { code: KeyCode::Escape, .. } => return true, // exit game
            Key { code: KeyCode::Enter, alt: true, .. } => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = tcod.root.is_fullscreen();
                tcod.root.set_fullscreen(!fullscreen);
            }

            // movement keys
            Key { code: KeyCode::Up, .. } => player.move_by(0, -1, &map),
            Key { code: KeyCode::Down, .. } => player.move_by(0, 1, &map),
            Key { code: KeyCode::Left, .. } => player.move_by(-1, 0, &map),
            Key { code: KeyCode::Right, .. } => player.move_by(1, 0, &map),

            _ => {}
        }

        false
    }

    pub fn run(&mut self) {
        system::set_fps(config::LIMIT_FPS);
        while !self.tcod.root.window_closed() {
            self.render_all();

            // handle keys and exit game if needed
            let player = &mut self.actors[0];
            if Game::handle_keys(&mut self.tcod, &self.map, player) {
                return;
            }
        }
    }
}