use tcod::{BackgroundFlag, Console, console, system};
use tcod::input::{Key, KeyCode};

use crate::{actors, map, screen};
use crate::config::{MAP_HEIGHT, MAP_WIDTH, COLOR_DARK_WALL, COLOR_DARK_GROUND, LIMIT_FPS, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO, COLOR_LIGHT_WALL, COLOR_LIGHT_GROUND};

pub struct Game {
    map: map::Map,
    actors: actors::Actors,
    tcod: screen::Tcod,
}

impl Game {
    pub fn new(tcod: screen::Tcod, map: map::Map, actors: actors::Actors) -> Self {
        Game { map, actors, tcod }
    }

    /// set the color and then draw the character that represents this object at its position
    fn draw_actor(tcod: &mut screen::Tcod, actor: &actors::Actor) {
        if tcod.fov.is_in_fov(actor.x(), actor.y()) {
            tcod.con.set_default_foreground(actor.color());
            tcod.con.put_char(actor.x(), actor.y(), actor.char(),
                                   BackgroundFlag::None);
        }
    }

    fn render_all(&mut self, fov_recompute: bool) {
        // clear the screen of the previous frame
        self.tcod.con.clear();

        if fov_recompute {
            // recompute FOV if needed (the player moved or something)
            self.tcod.fov.compute_fov(self.actors.player.x(),
                                      self.actors.player.y(),
                             TORCH_RADIUS,
                             FOV_LIGHT_WALLS,
                             FOV_ALGO);
        }

        // draw all objects in the list
        Game::draw_actor(&mut self.tcod, &self.actors.player);
        for actor in &self.actors.npcs {
            Game::draw_actor(&mut self.tcod, actor);
        }

        // go through all tiles, and set their background color
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let visible = self.tcod.fov.is_in_fov(x, y);
                let wall = self.map.blocks_sight(x, y);
                if visible {
                    // since it's visible, explore it
                    self.map.explore(x, y);
                }
                let color = match (visible, wall) {
                    // outside of field of view:
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,
                    // inside fov:
                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
                };
                if self.map.explored(x, y) {
                    // show explored tiles only (any visible tile is explored already)
                    self.tcod.con.set_char_background(x, y, color, BackgroundFlag::Set);
                }
            }
        }

        // blit the contents of "con" to the root console and present it
        console::blit(
            &self.tcod.con,
            (0, 0),
            (MAP_WIDTH, MAP_HEIGHT),
            &mut self.tcod.root,
            (0, 0),
            1.0,
            1.0,
        );

        self.tcod.root.flush();
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
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                self.tcod.fov.set(
                    x,
                    y,
                    !self.map.blocks_sight(x, y),
                    !self.map.is_passable(x, y),
                );
            }
        }
        let mut previous_player_position = (-1, -1);
        while !self.tcod.root.window_closed() {
            let fov_recompute = previous_player_position != (self.actors.player.x(), self.actors.player.y());
            self.render_all(fov_recompute);

            // handle keys and exit game if needed
            previous_player_position = (self.actors.player.x(), self.actors.player.y());
            if self.handle_keys() {
                return;
            }
        }
    }
}