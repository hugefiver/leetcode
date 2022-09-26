use std::collections::{HashMap, hash_map::Entry};

impl Solution {
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut start_indices = Vec::<i32>::new();
        if words.is_empty() {
            return start_indices;
        }

        let word_size = words[0].len();
        let window_size = word_size * words.len();

        if let Some(last_split) = s.len().checked_sub(window_size) {
            let mut word_set = HashMap::with_capacity(words.len());
            words.iter().for_each(|w| {
                let counter = word_set.entry(&w[..]).or_insert(0);
                *counter += 1;
            });
            word_set.shrink_to_fit();
            let mut seen = word_set.keys().map(|k| (*k, 0)).collect::<HashMap<_, _>>();
            for i in 0..word_size.min(last_split + 1) {
                let mut j = i + window_size;
                while j <= s.len() {
                    let mut k = 1;
                    while k <= words.len() {
                        let current_pos = j - k * word_size;
                        let current = &s[current_pos..(current_pos + word_size)];
                        match seen.entry(current) {
                            Entry::Occupied(entry) => {
                                let res = entry.into_mut();
                                *res += 1;
                                if *res > *word_set.get(current).unwrap() {
                                    break;
                                } else {
                                    k += 1;
                                }
                            }
                            Entry::Vacant(_) => {
                                break;
                            }
                        }
                    }

                    let start = j - window_size;
                    if k > words.len() {
                        start_indices.push(start as i32);
                        j += word_size;
                    } else {
                        j = window_size + (j - (k - 1) * word_size);
                    }
                    seen.values_mut().for_each(|v| *v = 0);
                }
            }
        }
        start_indices
    }

    pub fn find_substring_1(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut hp = HashMap::new();
        let n = words.len();
        let m = words[0].len();
        for word in words {
            hp.entry(word).and_modify(|w| *w += 1).or_insert(1);
        }
        for (idx, win) in s
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .windows(n * m)
            .enumerate()
        {
            let mut hp_only = hp.clone();
            for word in win.chunks(m) {
                hp_only
                    .entry(word.iter().collect::<String>())
                    .and_modify(|w| *w -= 1);
            }
            if hp_only.into_iter().all(|(_, v)| v == 0) {
                res.push(idx as i32);
            }
        }
        res
    }
}

struct Solution;

fn main() {}
