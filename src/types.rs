use self::Category::*;
use dice::{Dice, MyDice, DICE_NUM};
use std::slice::Iter;
use std::iter::Chain;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfAKind,
    FourOfAKind,
    FullHouse,
    SmallStraight,
    LargeStraght,
    Yahtzee,
    Chance,
}

pub type Value = u8;
pub type Entry = (Category, Value);

pub const FULL_HOUSE_VALUE: Value = 25;
pub const SMALL_STRAIGHT_VALUE: Value = 30;
pub const LARGE_STRAIGHT_VALUE: Value = 40;
pub const YAHTZEE_VALUE: Value = 50;

impl Category {
    const UPPER_CATEGORIES: [Category; 6] = [Ones, Twos, Threes, Fours, Fives, Sixes];
    const LOWER_CATEGORIES: [Category; 7] = [ThreeOfAKind, FourOfAKind, FullHouse,
                                              SmallStraight, LargeStraght, Yahtzee, Chance];

    pub fn is_lower(&self) -> bool {
        Category::LOWER_CATEGORIES.contains(self)
    }

    pub fn is_upper(&self) -> bool {
        Category::UPPER_CATEGORIES.contains(self)
    }

    pub fn upper_into_iter() -> Iter<'static, Category> {
        Category::UPPER_CATEGORIES.into_iter()
    }

    pub fn lower_into_iter() -> Iter<'static, Category> {
        Category::LOWER_CATEGORIES.into_iter()
    }

    pub fn into_iter() -> Chain<Iter<'static, Category>, Iter<'static, Category>>{
        Category::upper_into_iter().chain(Category::lower_into_iter())
    }
}

pub trait MyEntry {
    fn is_upper(&self) -> bool;
    fn upper_above_bonus(&self) -> i32;
}

impl MyEntry for Entry {
    fn is_upper(&self) -> bool {
        self.0.is_upper()
    }

    fn upper_above_bonus(&self) -> i32 {
        match self.0 {
            Ones => self.1 as i32 - 3 * 1,
            Twos => self.1 as i32 - 3 * 2,
            Threes => self.1 as i32 - 3 * 3,
            Fours => self.1 as i32 - 3 * 4,
            Fives => self.1 as i32 - 3 * 5,
            Sixes => self.1 as i32 - 3 * 6,
            _ => 0,
        }
    }
}

pub trait DiceCategory {
    fn is_category(&self, category: &Category) -> bool;
    fn value_of_category(&self, category: &Category) -> Value;
    fn entry_of_category(&self, category: Category) -> Entry;
    fn possible_entries(&self) -> HashMap<Category, u8>;
    fn possible_entries_sorted(&self) -> Vec<Entry>;
}

impl DiceCategory for Dice {
    fn is_category(&self, category: &Category) -> bool {
        match category {
            Ones => self.number_count(1) > 0,
            Twos => self.number_count(2) > 0,
            Threes => self.number_count(3) > 0,
            Fours => self.number_count(4) > 0,
            Fives => self.number_count(5) > 0,
            Sixes => self.number_count(6) > 0,
            ThreeOfAKind => is_three_of_a_kind(self),
            FourOfAKind => is_four_of_a_kind(self),
            FullHouse => is_full_house(self),
            SmallStraight => is_small_straight(self),
            LargeStraght => is_large_straight(self),
            Yahtzee => is_yahtzee(self),
            Chance => true,
        }
    }

    fn value_of_category(&self, category: &Category) -> Value {
        match category {
            Ones => self.number_count(1),
            Twos => self.number_count(2) * 2,
            Threes => self.number_count(3) * 3,
            Fours => self.number_count(4) * 4,
            Fives => self.number_count(5) * 5,
            Sixes => self.number_count(6) * 6,
            ThreeOfAKind => self.is_category(category) as Value * self.value(),
            FourOfAKind => self.is_category(category) as Value * self.value(),
            FullHouse => self.is_category(category) as Value * FULL_HOUSE_VALUE,
            SmallStraight => self.is_category(category) as Value * SMALL_STRAIGHT_VALUE,
            LargeStraght => self.is_category(category) as Value * LARGE_STRAIGHT_VALUE,
            Yahtzee => self.is_category(category) as Value * YAHTZEE_VALUE,
            Chance => self.is_category(category) as Value * self.value(),
        }
    }

    fn entry_of_category(&self, category: Category) -> Entry {
        let value = self.value_of_category(&category); 
        (category, value)
    }

    fn possible_entries(&self) -> HashMap<Category, u8> {
        let mut map = HashMap::new();
        for &cat in Category::into_iter() {
            map.insert(cat, self.value_of_category(&cat));
        }
        map
    }

    fn possible_entries_sorted(&self) -> Vec<Entry> {
        let mut res = self.possible_entries().iter().map(|(&c, &e)| (c, e)).collect::<Vec<_>>();
        res.sort_unstable_by(|a, b| a.1.cmp(&b.1).reverse());
        res
    }
}


fn is_three_of_a_kind(dice: &Dice) -> bool {
    for x in 1..DICE_NUM+1 {
        let x_count = dice.number_count(x);
        if x_count >= 3 {
            return true
        }
    }
    false
}

fn is_four_of_a_kind(dice: &Dice) -> bool {
    for x in 1..DICE_NUM+1 {
        let x_count = dice.number_count(x);
        if x_count >= 4 {
            return true
        }
    }
    false
}

fn is_full_house(dice: &Dice) -> bool {
    for x in 1..6 {
        for y in x+1..=6 {
            let x_count = dice.number_count(x);
            let y_count = dice.number_count(y);
            if (x_count >= 3 && y_count >= 2) || (y_count >= 3 && x_count >= 2) {
                return true
            }
        }
    }
    false
}

fn is_small_straight(dice: &Dice) -> bool {
    dice.number_count(3) >= 1 && dice.number_count(4) >= 1
        && ((dice.number_count(1) >= 1 && dice.number_count(2) >= 1)
            || (dice.number_count(2) >= 1 && dice.number_count(5) >= 1)
            || (dice.number_count(5) >= 1 && dice.number_count(6) >= 1))
}

fn is_large_straight(dice: &Dice) -> bool {
    dice.number_count(2) == 1
        && dice.number_count(3) == 1
        && dice.number_count(4) == 1
        && dice.number_count(5) == 1
        && (dice.number_count(1) == 1 || dice.number_count(6) == 1)
}

fn is_yahtzee(dice: &Dice) -> bool {
    for x in 1..=6 {
        let x_count = dice.number_count(x);
        if x_count == DICE_NUM {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    fn full_houses() -> Vec<Dice> {
        let mut full_houses = Vec::new();
        for x in 1..=6 {
            for y in x+1..=6 {
                full_houses.push(vec![x,x,x,y,y]);
                full_houses.push(vec![x,x,y,y,y]);
            }
        }
        full_houses
    }

    fn small_straights() -> Vec<Dice> {
        let straights = vec![vec![1,2,3,4], vec![2,3,4,5], vec![3,4,5,6]];
        let mut small_straights = Vec::new();
        for i in 1..=6 {
            for mut s in straights.iter().cloned() {
                //dont make large_straights
                if s.contains(&i) {
                    s.push(i);
                    small_straights.push(s);
                }
            }
        }
        small_straights.push(vec![1,2,3,4,6]);
        small_straights.push(vec![1,3,4,5,6]);
        small_straights
    }

    fn large_straights() -> Vec<Dice> {
        vec![vec![1,2,3,4,5], vec![2,3,4,5,6]]
    }

    fn yahtzees() -> Vec<Dice> {
        let mut yahtzees = Vec::new();
        for i in 1..=6 {
            yahtzees.push(iter::repeat(i).take(DICE_NUM as usize).collect());
        }
        yahtzees
    }

    #[test]
    fn test_is_full_house() {
        for i in full_houses() {
            assert!(is_full_house(&i));
        }
        for i in small_straights() {
            assert!(!is_full_house(&i));
        }
        for i in large_straights() {
            assert!(!is_full_house(&i));
        }
        for i in yahtzees() {
            assert!(!is_full_house(&i));
        }
        assert!(!is_full_house(&vec![2,2,3,4,5]));
        assert!(!is_full_house(&vec![2,3,4,4,5]));
        assert!(!is_full_house(&vec![2,3,4,5,5]));
        assert!(!is_full_house(&vec![1,2,3,4,5]));
        assert!(!is_full_house(&vec![2,3,4,5,6]));
        assert!(!is_full_house(&vec![2,6,4,5,3]));
    }

    #[test]
    fn test_is_small_straight() {
        for i in large_straights() {
            assert!(is_small_straight(&i));
        }
        for i in small_straights() {
            assert!(is_small_straight(&i));
        }
        for i in full_houses() {
            assert!(!is_small_straight(&i));
        }
        for i in yahtzees() {
            assert!(!is_small_straight(&i));
        }
        assert!(!is_small_straight(&vec![1,1,1,1,5]));
        assert!(!is_small_straight(&vec![5,1,1,1,1]));
    }

    #[test]
    fn test_is_large_straight() {
        for i in large_straights() {
            assert!(is_large_straight(&i));
        }
        for i in full_houses() {
            assert!(!is_large_straight(&i));
        }
        for i in small_straights() {
            assert!(!is_large_straight(&i));
        }
        for i in yahtzees() {
            assert!(!is_large_straight(&i));
        }
        assert!(is_large_straight(&vec![2,6,4,5,3]));
        assert!(!is_large_straight(&vec![2,3,4,4,5]));
        assert!(!is_large_straight(&vec![2,3,4,5,5]));
        assert!(!is_large_straight(&vec![1,1,1,1,5]));
        assert!(!is_large_straight(&vec![5,1,1,1,1]));
    }

    #[test]
    fn test_is_yahtzee() {
        for i in yahtzees() {
            assert!(is_yahtzee(&i));
        }
        for i in full_houses() {
            assert!(!is_yahtzee(&i));
        }
        for i in small_straights() {
            assert!(!is_yahtzee(&i));
        }
        for i in large_straights() {
            assert!(!is_yahtzee(&i));
        }
        assert!(!is_yahtzee(&vec![1,1,1,1,5]));
        assert!(!is_yahtzee(&vec![5,1,1,1,1]));
    }
}
