use types::{Category, DiceCategory, Entry, MyEntry};
use types::Category::*;
use dice::{Dice, MyDice, DICE_NUM, subsets, all_tosses};
use player::Player;
use result::{Result, BONUS_REACH, BONUS_VALUE};
use std::cmp::Ordering;

pub struct YahtzeeAI {
    name: String,
}

impl YahtzeeAI {
    pub fn new() -> YahtzeeAI {
        YahtzeeAI { name: String::from("KI") }
    }
}

impl Player for YahtzeeAI {
    fn decide_keep(&self, result: &Result, remaining_tosses: u8, dice: &Dice) -> Dice {
        let mut keeps: Vec<(Dice, f32)> = subsets(dice).into_iter().map(
            |subset| {
                let eval = YahtzeeAI::evaluate_keep(result, remaining_tosses, dice, &subset); 
                print!("{:?}, {} - ", subset, eval);
                (subset, eval)
            }
        ).collect();
        println!();
        keeps.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less).reverse());
        keeps.remove(0).0
    }

    fn decide_entry(&self, result: &Result, dice: &Dice) -> Entry {
        let mut entries: Vec<_> = Category::into_iter().filter(|&&cat| !result.has(cat)).map(|&cat| {
            let entry = dice.entry_of_category(cat);
            let eval = YahtzeeAI::evaluate_entry(result, &entry); 
            print!("{:?}, {} - ", entry, eval);
            (entry, eval)
        }).collect();
        println!();
        entries.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less).reverse());
        entries.remove(0).0
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl YahtzeeAI {
    fn evaluate_keep(result: &Result, remaining_tosses: u8, dice: &Dice, keep: &Dice) -> f32 {
        let mut value = 0f32;
        let tosses = all_tosses(DICE_NUM - keep.len() as u8); 
        let mut tosses_num = 1;
        value += YahtzeeAI::evaluate_value(result, &dice) as f32;
        for (toss, times) in tosses.iter() {
            let mut future_dice = keep.clone();
            future_dice.extend(toss);
            value += YahtzeeAI::evaluate_value(result, &future_dice) * *times as f32;
            tosses_num += times;
        }
        value / tosses_num as f32
    }

    fn evaluate_value(result: &Result, dice: &Dice) -> f32 {
        let mut max = 0f32;
        for &cat in Category::into_iter() {
            if result.has(cat) || cat == Category::Chance {
                continue
            }
            let value = YahtzeeAI::evaluate_entry(result, &dice.entry_of_category(cat));
            if value > max {
                max = value.into();
            }
        }
        max
    }

    fn evaluate_entry(result: &Result, entry: &Entry) -> f32 {
        let value = entry.1 as f32;
        if entry.is_upper() && !result.has_bonus() {
            let above_bonus_points = entry.upper_above_bonus() as f32;
            // let bonus_to_go = BONUS_REACH - result.value_upper();
            // let needed_percent = value / bonus_to_go as f32;
            // value + needed_percent * BONUS_VALUE as f32
            value + above_bonus_points
        } else {
            value
        }
    }
}

pub fn probability_dice(dice_num: u8, wanted_dice: &Dice, tries: u8) -> f32 {
    if dice_num < wanted_dice.len() as u8 {
        return 0f32;
    }
    unimplemented!()
}
