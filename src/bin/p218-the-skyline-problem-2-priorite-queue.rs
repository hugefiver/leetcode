use std::collections::{BTreeSet, BinaryHeap};

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut bh: BinaryHeap<(i32, i32, i32)> = BinaryHeap::with_capacity(buildings.len());
        let mut points: BTreeSet<i32> = BTreeSet::new();
        for building in buildings.iter() {
            points.insert(building[0]);
            points.insert(building[1]);
        }
        let (mut i, mut h) = (0, 0);
        let mut answer = Vec::new();
        for &x in points.iter() {
            while i < buildings.len() && buildings[i][0] == x {
                bh.push((buildings[i][2], buildings[i][0], buildings[i][1]));
                i += 1;
            }
            while let Some(top) = bh.peek() {
                if top.2 <= x {
                    bh.pop();
                } else {
                    break;
                }
            }
            if let Some(top) = bh.peek() {
                if top.0 != h {
                    answer.push(vec![x, top.0]);
                    h = top.0;
                }
            } else {
                answer.push(vec![x, 0]);
            }
        }
        answer
    }
}
struct Solution;

fn main() {}
