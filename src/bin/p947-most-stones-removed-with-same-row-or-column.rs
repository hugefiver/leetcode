use std::collections::{HashMap, HashSet};

pub struct DSU {
    parents: HashMap<i32, i32>,
    ranks: HashMap<i32, i32>,
}

impl DSU {
    pub fn new() -> Self {
        DSU {
            parents: HashMap::new(),
            ranks: HashMap::new(),
        }
    }

    pub fn parent(&mut self, x: i32) -> &mut i32 {
        self.parents.entry(x).or_insert(x)
    }

    pub fn rank(&mut self, x: i32) -> &mut i32 {
        self.ranks.entry(x).or_insert(0)
    }

    pub fn find(&mut self, x: i32) -> i32 {
        if self.parent(x) != &x {
            let p = *self.parent(x);
            *self.parent(x) = self.find(p);
        }
        *self.parent(x)
    }

    pub fn union(&mut self, x: i32, y: i32) {
        let (x_rank, y_rank) = (self.find(x), self.find(y));
        if x_rank != y_rank {
            use std::cmp::Ordering::*;
            let (x_parent, y_parent) = (*self.parent(x_rank), *self.parent(y_rank));
            match x_parent.cmp(&y_parent) {
                Less => *self.parent(x_rank) = y_rank,
                Greater => *self.parent(y_rank) = x_rank,
                Equal => {
                    *self.parent(y_rank) = x_rank;
                    *self.rank(x_rank) += 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut dsu = DSU::new();
        let mut set = HashSet::new();
        for stone in stones.iter() {
            dsu.union(stone[0] * 2, stone[1] * 2 + 1);
        }
        for stone in stones.iter() {
            set.insert(dsu.find(stone[0] * 2));
        }
        (stones.len() - set.len()) as i32
    }
}

struct Solution;

fn main() {}
