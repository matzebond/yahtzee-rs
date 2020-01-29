extern crate rand;

mod ai;
mod dice;
mod game;
mod player;
mod result;
mod types;

use ai::YahtzeeAI;
use game::Game;
use rand::FromEntropy;
use rand::rngs::SmallRng;

fn main() {
    let mut game = Game::new(vec![YahtzeeAI::new()], SmallRng::from_entropy());
    game.run();
}
