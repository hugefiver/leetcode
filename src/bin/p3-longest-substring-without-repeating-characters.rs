impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        if s.len() <= 1 {
            return s.len() as i32;
        }
        let xs: Vec<_> = s.chars().collect();
        let mut map = HashMap::new();

        let (mut i, mut j) = (0, 0);
        map.insert(xs[0], 1);
        let mut max = 1;
        while j < xs.len() - 1 {
            if map.get(&xs[j + 1]).cloned().unwrap_or(0) == 0 && map.values().all(|&x| x <= 1){
                j += 1;
                map.insert(xs[j], 1);
                if map.len() > max {
                    max = map.values().filter(|x| **x == 1).count();
                }
            } else {
                while j < xs.len() - 1
                    && (map.get(&xs[j + 1]).unwrap_or(&0).ne(&0) || map.values().any(|&x| x > 1))
                {
                    *map.get_mut(&xs[i]).unwrap() -= 1;
                    j += 1;
                    i += 1;
                    match map.get_mut(&xs[j]) {
                        Some(val) => *val += 1,
                        None => {
                            map.insert(xs[j], 1);
                        }
                    };
                }
            }
        }
        max as i32
    }
}

struct Solution;

fn main() {}
