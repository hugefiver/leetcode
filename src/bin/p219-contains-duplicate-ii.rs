use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use Entry::*;

        let mut map = HashMap::new();
        nums.into_iter().enumerate().any(|(i, x)| {
            let v = map.entry(x);
            match v {
                Occupied(mut o) => {
                    let j = o.get();
                    if i - j <= k as usize {
                        true
                    } else {
                        o.insert(i);
                        false
                    }
                }
                Vacant(v) => {
                    v.insert(i);
                    false
                }
            }
        })
    }
}

impl Solution1 {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() <= 1 || k == 0 {
            false
        } else if nums.len() <= k as usize {
            let len = nums.len();
            nums.into_iter().collect::<HashSet<_>>().len() != len
        } else {
            let (init, rest) = nums.split_at(k as usize);
            let mut set: HashSet<_> = init.iter().collect();
            if set.len() < k as usize {
                true
            } else {
                nums.iter()
                    .zip(rest.iter())
                    .map(|(x, y)| {
                        if set.contains(y) {
                            true
                        } else {
                            set.remove(x);
                            set.insert(y);
                            false
                        }
                    })
                    .any(|x| x)
            }
        }
    }
}

struct Solution;
struct Solution1;

fn main() {}
