use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut fulls : HashMap<&[u8], i32> = HashMap::new();
        let wl = words.len();
        
        let mut min_len = 1000; // words[i].length < 300, wo should be enough
        let mut max_len = 0;
        for i in 0..wl {
            min_len = usize::min(min_len, words[i].len());
            max_len = usize::max(max_len, words[i].len());
        }
        
        for i in 0..wl {
            let word = &(words[i as usize]);
            let bs = word.as_bytes();
            fulls.insert(&bs[..], i as i32);
        }
        
        let mut res : Vec<Vec<i32>> = Vec::new();
        for i in 0..wl {
            let word = &(words[i as usize]);
            let bs = word.as_bytes();
            let mut bsr = word.clone().into_bytes();
            bsr.reverse();
            let bsr = bsr;
            
            if let Some(x) = fulls.get(&bsr[..]) {
                if (*x != i as i32) {
                    res.push(vec![i as i32, *x]);
                }
            }
            
            let l = bs.len();
            let min_keep = min_len;
            let max_keep = usize::min(l - 1, max_len);
            
            for d in (l - max_keep)..(l - min_keep + 1) {
                let j = d / 2;
                if let Some(x) = fulls.get(&bsr[d..]) {
                    if bsr[..j] == bs[(l - d)..(l - d + j)] {
                        res.push(vec![i as i32, *x]);
                    }
                }
                if let Some(x) = fulls.get(&bsr[..(l - d)]) {
                    let j = d / 2;
                    if bsr[(l - d)..(l - d + j)] == bs[..j] {
                        res.push(vec![*x, i as i32]);
                    }
                }
            }
        }
        
        res
    }
}

struct Solution;

fn main() {}
