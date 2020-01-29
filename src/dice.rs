use types::Value;
use std::cmp::Ordering;

pub const DICE_NUM: u8 = 5;
pub const DICE_FACES: [Die; 6] = [1,2,3,4,5,6];

pub type Die = u8;
pub type Dice = Vec<Die>;

pub trait MyDice {
    fn value(&self) -> Value;
    fn number_count(&self, number: Die) -> u8;
    fn keep_subset(&self, other: &Dice) -> Dice;
}

impl MyDice for Dice {
    fn value(&self) -> Value {
        self.iter().sum()
    }

    fn number_count(&self, number: Die) -> u8 {
        self.iter().filter(|&&d| d == number).count() as u8
    }

    fn keep_subset(&self, other: &Dice) -> Dice {
        let mut new_dice = Dice::new();
        let mut other_iter = other.iter();
        let mut cur_other = 0;
        for &d in self {
            while cur_other < d {
                cur_other = match other_iter.next() {
                    Some(&k) => k,
                    _ => 7
                }
            }
            if cur_other == d {
                new_dice.push(d);
            }
        }
        new_dice
    }
}

pub fn is_permutation<T: Clone + Ord>(a: &[T], b: &[T]) -> bool {
    let mut this = a.to_vec();
    let mut other = b.to_vec();
    this.sort_unstable();
    other.sort_unstable();
    this == other
}

///both sorted
pub fn is_subset<T: Ord>(subset: &Vec<T>, superset: &Vec<T>) -> bool {
    let mut sub_iter = subset.iter();
    let mut cur_a = sub_iter.next();
    for b in superset {
        if let Some(a) = cur_a {
            if a == b {
                cur_a = sub_iter.next()
            } else if a < b {
                return false;
            }
            continue;
        }
    }
    return cur_a.is_none();
}

pub fn subsets<T: Clone + Ord>(orig: &[T]) -> Vec<Vec<T>> {
    let mut this = orig.to_vec();
    this.reverse();
    let mut subsets = _subsets(&this);
    subsets.sort_unstable_by(|a, b| cmp_by_length(a, b));
    let mut result = Vec::new();
    let mut last = subsets[0].clone();
    for sub in subsets {
        if sub != last {
            result.push(last);
            last = sub;
        }
    }
    result.push(last);
    result
}

fn _subsets<T: Clone>(orig: &[T]) -> Vec<Vec<T>> {
    if orig.len() == 0 {
        vec![vec![]]
    } else {
        let mut result = Vec::new();
        for mut subset in _subsets(&orig[1..]) {
            result.push(subset.clone());
            subset.push(orig[0].clone());
            result.push(subset);
        }
        result
    }
}

fn cmp_by_length<T: Ord>(a: &[T], b: &[T]) -> Ordering {
    if a.len() > b.len() {
        Ordering::Greater
    } else if a.len() < b.len() {
        Ordering::Less
    } else {
        a.cmp(b)
    }
}

pub fn variations_with_repetition<T: Clone>(orig: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        vec![vec![]]
    } else {
        let mut result = Vec::new();
        for comb in variations_with_repetition(orig, size-1) {
            for elem in orig {
                let mut full_comb = comb.clone();
                full_comb.push(elem.clone());
                result.push(full_comb);
            }
        }
        result
    }
}

pub fn variations_with_repetition_hist<T: Clone + Ord>(orig: &[T], size: usize) -> Vec<(Vec<T>, usize)> {
    let mut variations = variations_with_repetition(orig, size);
    for var in variations.iter_mut() {
        var.sort_unstable();
    }
    variations.sort_unstable();
    let mut history = Vec::new();
    let mut last = variations[0].clone();
    let mut cnt = 0;
    for var in variations {
        if var != last {
            history.push((last, cnt));
            last = var;
            cnt = 1;
        }
        else {
            cnt += 1;
        }
    }
    history
}

pub fn all_tosses(num_dice: u8) -> Vec<(Dice, usize)> {
    variations_with_repetition_hist(&DICE_FACES , num_dice as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        let ref vec = vec![1, 2, 5, 5, 6];
        assert!(is_permutation(vec, &vec![1,2,5,5,6]));
        assert!(is_permutation(vec, &vec![2,1,5,5,6]));
        assert!(is_permutation(vec, &vec![5,2,1,5,6]));
        assert!(is_permutation(vec, &vec![5,2,5,1,6]));
        assert!(is_permutation(vec, &vec![6,2,5,5,1]));
        assert!(is_permutation(vec, &vec![1,5,2,5,6]));
        assert!(is_permutation(vec, &vec![1,5,5,2,6]));
        assert!(is_permutation(vec, &vec![1,6,5,5,2]));
    }

    #[test]
    fn test_is_subset() {
        let ref super_set = vec![1, 2, 5, 5, 6];
        assert!(is_subset(&vec![], super_set));
        assert!(is_subset(&vec![1], super_set));
        assert!(is_subset(&vec![1, 5], super_set));
        assert!(is_subset(&vec![1, 5, 5], super_set));
        assert!(is_subset(&vec![1, 5, 6], super_set));
        assert!(is_subset(&vec![2, 5, 5], super_set));
        assert!(is_subset(&vec![2, 5, 6], super_set));
        assert!(is_subset(&vec![5], super_set));
        assert!(is_subset(&vec![6], super_set));
    }

    #[test]
    fn test_subsets() {
        let ref vec = vec![1, 2, 5, 5, 6];
        assert!(subsets(vec).contains(vec));
        assert!(subsets(vec).contains(&vec![1]));
        assert!(subsets(vec).contains(&vec![1, 2]));
        assert!(subsets(vec).contains(&vec![1, 5]));
        assert!(subsets(vec).contains(&vec![5]));
        assert!(subsets(vec).contains(&vec![5, 5]));
        assert!(subsets(vec).contains(&vec![2, 6]));
        assert!(subsets(vec).contains(&vec![1, 2, 5]));
        assert!(subsets(vec).contains(&vec![5, 5, 6]));
    }


    #[test]
    fn test_subsets_size_3() {
        let vec = vec![1,2,3];
        let subs = vec![vec![], vec![1], vec![2], vec![3], vec![1,2],
                        vec![1,3], vec![2,3], vec![1,2,3]];
        
        assert!(is_permutation(&subsets(&vec), &subs));
    }

    #[test]
    fn test_subsets_size_4_with_double() {
        let ref vec = vec![1,2,3,3];
        let ref subs = vec![vec![], vec![1], vec![2], vec![3], vec![1,2],
                        vec![1,3], vec![2,3], vec![3,3], vec![1,2,3],
                        vec![1,3,3], vec![2,3,3], vec![1,2,3,3]];
        println!("{:?}", subsets(vec));
        assert!(is_permutation(&subsets(vec), subs));
    }

    #[test]
    fn test_subsets_is_subset() {
        let vec = vec![1, 2, 3, 4, 5];
        for ref subset in subsets(&vec) {
            println!("{:?}", subset);
            assert!(is_subset(subset, &vec));
        }
    }

    #[test]
    fn test_variation_with_repetition() {
        let variations = variations_with_repetition(&DICE_FACES, 3);
        unimplemented!()
    }

    #[test]
    fn test_variation_with_repetition_hist() {
        let variations = variations_with_repetition_hist(&DICE_FACES, 3);
        println!("{:?}", variations);
        assert!(variations.contains(&(vec![1,1,2], 3)));
        assert!(variations.contains(&(vec![1,1,5], 3)));
        assert!(variations.contains(&(vec![1,3,5], 6)));
    }
}
