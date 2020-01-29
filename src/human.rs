use types::{Dice, Entry};
use player::Player;

pub struct Human {
    name: &str,
}

impl Human {
    pub fn new(name: &str) {
        Human { name }
    }
}

impl Player for Human {
    fn decide_keep(&self, round: &Round) -> Dice {
    }

    fn decide_entry(&self, dices: &Dice) -> Entry {
    }
}
