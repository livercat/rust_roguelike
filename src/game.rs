use tcod::{system};
use tcod::input::{Key, KeyCode};

use crate::{actors, map, renderer};
use crate::config::LIMIT_FPS;

pub struct Game {
    map: map::Map,
    actors: actors::Actors,
    tcod: renderer::Tcod,
}

impl Game {
    pub fn new(tcod: renderer::Tcod, map: map::Map, actors: actors::Actors) -> Self {
        Game { map, actors, tcod }
    }

    fn handle_keys(&mut self) -> bool {
        let key = self.tcod.root.wait_for_keypress(true);
        match key {
            Key { code: KeyCode::Escape, .. } => return true, // exit game
            Key { code: KeyCode::Enter, alt: true, .. } => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = self.tcod.root.is_fullscreen();
                self.tcod.root.set_fullscreen(!fullscreen);
            }

            // movement keys
            Key { code: KeyCode::Up, .. } => self.actors.player.move_by(0, -1, &self.map),
            Key { code: KeyCode::Down, .. } => self.actors.player.move_by(0, 1, &self.map),
            Key { code: KeyCode::Left, .. } => self.actors.player.move_by(-1, 0, &self.map),
            Key { code: KeyCode::Right, .. } => self.actors.player.move_by(1, 0, &self.map),

            _ => {}
        }

        false
    }

    pub fn run(&mut self) {
        system::set_fps(LIMIT_FPS);
        self.tcod.init_fov(&self.map);
        let mut prev_player_pos = (-1, -1);

        while !self.tcod.root.window_closed() {
            let recompute_fov = prev_player_pos != (self.actors.player.x(), self.actors.player.y());
            self.tcod.render_all(&mut self.map, &self.actors, recompute_fov);

            // handle keys and exit game if needed
            prev_player_pos = (self.actors.player.x(), self.actors.player.y());
            if self.handle_keys() {
                return;
            }
        }
    }
}