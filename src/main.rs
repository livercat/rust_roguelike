use tcod::colors;

use roguelike::actors;
use roguelike::game;
use roguelike::map;
use roguelike::screen;

fn main() {
    // create object representing the player
    let player = actors::Actor::new(
        0,
        0,
        '@', colors::WHITE);

    // create an NPC
    let npc = actors::Actor::new(
        0,
        0,
        '@', colors::YELLOW);

    // the list of objects with those two
    let mut actors = vec![player, npc];
    let tcod = screen::Tcod::new();
    let map = map::make_map(&mut actors);
    let mut game = game::Game::new(tcod, map, actors);

    game.run();
}