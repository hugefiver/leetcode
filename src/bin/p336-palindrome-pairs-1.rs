use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut rev_fulls : HashMap<&[u8], i32> = HashMap::new();
        let mut rev_prefixes : HashMap<&[u8], Vec<i32>> = HashMap::new();
        let mut rev_suffixes : HashMap<&[u8], Vec<i32>> = HashMap::new();
        let wl = words.len();
        
        let mut min_len = 1000; // words[i].length < 300, wo should be enough
        let mut max_len = 0;
        for i in 0..wl {
            min_len = usize::min(min_len, words[i].len());
            max_len = usize::max(max_len, words[i].len());
        }
        // println!("{}-{}", min_len, max_len);
        
        for i in 0..wl {
            let word = &(words[i as usize]);
            let bs = word.as_bytes();
            rev_fulls.insert(&bs[..], i as i32);
            
            let l = bs.len();
            let min_keep = min_len;
            let max_keep = usize::min(l - 1, max_len);
            // println!("{}, {}-{}", word, min_keep, max_keep);
            for d in (l - max_keep)..(l - min_keep + 1) {
                let mut ok = true;
                for j in 0..(d / 2) {
                    if bs[l - 1 - j] != bs[l - d + j] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                rev_prefixes.entry(&bs[..(l-d)]).and_modify(|x| x.push(i as i32)).or_insert(vec![i as i32]);
            }
            
            for d in (l - max_keep)..(l - min_keep + 1) {
                let mut ok = true;
                for j in 0..(d / 2) {
                    if bs[j] != bs[d - 1 - j] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                rev_suffixes.entry(&bs[d..]).and_modify(|x| x.push(i as i32)).or_insert(vec![i as i32]);
            }
        }
        
        // println!("{:?}", rev_fulls);
        // println!("{:?}", rev_prefixes);
        // println!("{:?}", rev_suffixes);
        
        let mut res : Vec<Vec<i32>> = Vec::new();
        
        for i in 0..(wl as i32) {
            let word = &(words[i as usize]);
            let mut bytes = word.clone().into_bytes();
            bytes.reverse();
            if let Some(j) = rev_fulls.get(&bytes[..]) {
                if *j != i {
                    res.push(vec![*j, i]);
                }
            }
            if let Some(t) = rev_prefixes.get(&bytes[..]) {
                for j in t {
                    if *j != i {
                        res.push(vec![*j, i]);
                    }
                }
            }
            if let Some(t) = rev_suffixes.get(&bytes[..]) {
                for j in t {
                    if *j != i as i32 {
                        res.push(vec![i, *j]);
                    }
                }
            }
        }
        
        res
    }
}

struct Solution;

fn main() {}
