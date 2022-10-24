impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut max = 0;

        let bit_sets: Vec<u32> = arr.into_iter().filter_map(|s| {
            let mut bit_set = 0;
            for c in s.chars() {
                let bit = 1 << (c as u8 - b'a');
                if bit_set & bit != 0 {
                    return None;
                }
                bit_set |= bit;
            }
            Some(bit_set)
        }).collect();

        let mut stack = vec![(0u32, 0)];
        while let Some((bit_set, idx)) = stack.pop() {
            max = max.max(bit_set.count_ones());
            for (i, &bit) in bit_sets.iter().enumerate().skip(idx) {
                if bit_set & bit == 0 {
                    stack.push((bit_set | bit, i + 1));
                }
            }
        }

        max as i32
    }
}

struct Solution;

fn main() {}
