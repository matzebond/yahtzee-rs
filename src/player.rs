use dice::Dice;
use types::Entry;
use result::Result;

pub trait Player {
    fn decide_keep(&self, result: &Result, remaining_tosses: u8, dice: &Dice) -> Dice;
    fn decide_entry(&self, result: &Result, dices: &Dice) -> Entry;
    fn name(&self) -> &str;
}
