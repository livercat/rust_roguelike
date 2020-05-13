use tcod::colors;

use roguelike::{actors, game, map, renderer};

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
    let mut actors = actors::Actors{player, npcs: vec![npc]};
    let tcod = renderer::Tcod::new();
    let map = map::Map::new(&mut actors);
    let mut game = game::Game::new(tcod, map, actors);

    game.run();
}