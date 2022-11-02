use std::collections::HashSet;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut hash: HashSet<Vec<u8>> =
            bank.into_iter().map(|s| s.as_bytes().to_owned()).collect();
        if !hash.contains(end.as_bytes()) {
            return -1;
        }
        let types = [b'A', b'C', b'G', b'T'];
        let mut curr = vec![start.as_bytes().to_owned()];
        let mut next = Vec::new();
        let mut step = 0;
        while !curr.is_empty() {
            for mut v in curr.drain(..) {
                if v == end.as_bytes() {
                    return step;
                }
                for i in 0..v.len() {
                    let b = v[i];
                    for &t in types.iter() {
                        if t != b {
                            v[i] = t;
                            if let Some(v2) = hash.take(&v) {
                                next.push(v2);
                            }
                        }
                    }
                    v[i] = b;
                }
            }
            std::mem::swap(&mut curr, &mut next);
            step += 1;
        }
        -1
    }

    pub fn min_mutation_2(start: String, end: String, bank: Vec<String>) -> i32 {
        if !bank.contains(&end) {
            return -1;
        }
        let mut visited = vec![false; bank.len()];
        let mut curr = vec![start.as_bytes()];
        let mut next = Vec::new();
        let mut step = 0;
        while !curr.is_empty() {
            for v in curr.drain(..) {
                if v == end.as_bytes() {
                    return step;
                }
                for i in 0..bank.len() {
                    if !visited[i] && Self::is_adj(v, bank[i].as_bytes()) {
                        next.push(bank[i].as_bytes());
                        visited[i] = true;
                    }
                }
            }
            std::mem::swap(&mut curr, &mut next);
            step += 1;
        }
        -1
    }

    fn is_adj(v1: &[u8], v2: &[u8]) -> bool {
        v1.iter()
            .zip(v2.iter())
            .filter(|&(&b1, &b2)| b1 == b2)
            .count()
            == 7
    }
}

struct Solution;

fn main() {}
