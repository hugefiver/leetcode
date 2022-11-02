use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let bank: HashSet<String> = bank.into_iter().chain(vec![start.clone()]).collect();

        if !bank.contains(&end) {
            return -1;
        }

        let (mut seen_left, mut seen_right) = (HashMap::new(), HashMap::new());
        let (mut q1, mut q2) = (VecDeque::new(), VecDeque::new());
        q1.push_back((start, 0));
        q2.push_back((end, 0));

        while q1.len() > 0 || q2.len() > 0 {
            if let Some((a, steps_left)) = q1.pop_front() {
                for b in bank
                    .iter()
                    .filter(|s| !seen_left.contains_key(*s) && s != &&a)
                {
                    if Self::adjacent(a.as_str(), b.as_str()) {
                        if let Some(steps_right) = seen_right.get(b) {
                            return steps_left + 1 + steps_right;
                        }

                        q1.push_back((b.clone(), steps_left + 1));
                    }
                }

                seen_left.insert(a, steps_left);
            }

            if let Some((a, steps_right)) = q2.pop_front() {
                for b in bank
                    .iter()
                    .filter(|s| !seen_right.contains_key(*s) && s != &&a)
                {
                    if Self::adjacent(a.as_str(), b.as_str()) {
                        if let Some(steps_left) = seen_left.get(b) {
                            return steps_left + 1 + steps_right;
                        }

                        q2.push_back((b.clone(), steps_right + 1));
                    }
                }

                seen_right.insert(a, steps_right);
            }
        }

        -1
    }

    fn adjacent(a: &str, b: &str) -> bool {
        let mut diff = 0;
        let (mut a_chars, mut b_chars) = (a.chars(), b.chars());

        while let (Some(a), Some(b)) = (a_chars.next(), b_chars.next()) {
            if a != b {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
        }

        true
    }
}

struct Solution;

fn main() {}
