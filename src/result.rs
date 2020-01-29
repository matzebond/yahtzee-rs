use types::{Category, Entry, MyEntry, Value};
use std::collections::HashMap;

pub const BONUS_REACH: u32 = 63;
pub const BONUS_VALUE: u32 = 35;

pub struct Result {
    map: HashMap<Category, Value>,
    bonus: bool,
}

impl Result {
    pub fn new() -> Result {
        Result { map: HashMap::new(), bonus: false }
    }

    pub fn has_bonus(&self) -> bool {
        self.value_upper() >= BONUS_REACH
    }

    pub fn bonus_progress(&self) -> i32 {
        let mut progress = 0;
        for (&cat, &val) in self.map.iter() {
            progress += (cat, val).upper_above_bonus()
        }
        progress
    }

    pub fn value_upper(&self) -> u32 {
        let mut value: u32 = 0;
        for &cat in Category::upper_into_iter() {
            if let Some(&v) = self.map.get(&cat) {
                value += v as u32
            }
        }
        value
    }

    pub fn value_lower(&self) -> u32 {
        let mut value: u32 = 0;
        for &cat in Category::lower_into_iter() {
            if let Some(&v) = self.map.get(&cat) {
                value += v as u32
            }
        }
        value
    }

    pub fn value(&self) -> u32 {
        let mut value = self.value_upper();
        if value >= BONUS_REACH {
            value += BONUS_VALUE
        }
        value += self.value_lower();
        value
    }

    pub fn add(&mut self, entry: Entry) {
        let (cat, val) = entry;
        self.map.insert(cat, val);
        self.bonus = self.value_upper() >= BONUS_REACH;
    }

    pub fn has(&self, category: Category) -> bool {
        self.map.contains_key(&category)
    }

    pub fn get(&self, category: Category) -> Option<u8> {
        self.map.get(&category).map(|&u| u)
    }
}
