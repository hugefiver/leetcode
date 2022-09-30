use std::collections::BinaryHeap;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut skyline = Vec::with_capacity(buildings.len() * 2);
        let mut active = BinaryHeap::<Building>::with_capacity(buildings.len());
        let mut buildings = buildings.iter().map(|b| Building { b }).peekable();

        while let Some(curr_x) = buildings.peek().map(|b| b.left()) {
            // deal with buildings that end before current x
            while active.peek().map_or(false, |b| b.right() < curr_x) {
                pop_rights_and_update(&mut active, &mut skyline);
            }
            // add all buildings with same left endpoint
            while let Some(b) = buildings.next_if(|b| b.left() == curr_x) {
                active.push(b);
            }
            update_skyline(curr_x, max_active_height(&active), &mut skyline);
        }

        while !active.is_empty() {
            pop_rights_and_update(&mut active, &mut skyline);
        }

        skyline
    }
}

fn pop_rights_and_update(active: &mut BinaryHeap<Building>, skyline: &mut Vec<Vec<i32>>) {
    let tallest_right = active.pop().unwrap().right();

    while active.peek().map_or(false, |b| b.right() <= tallest_right) {
        active.pop();
    }

    update_skyline(tallest_right, max_active_height(active), skyline)
}

fn max_active_height(active: &BinaryHeap<Building>) -> i32 {
    active.peek().map_or(0, |b| b.height())
}

fn update_skyline(x: i32, y: i32, skyline: &mut Vec<Vec<i32>>) {
    let last = skyline.last();
    if last == None || y != last.unwrap()[1] {
        skyline.push(vec![x, y]);
    }
}

#[derive(PartialEq, Eq)]
struct Building<'a> {
    b: &'a Vec<i32>,
}

impl<'a> Building<'a> {
    fn left(&self) -> i32 {
        self.b[0]
    }

    fn right(&self) -> i32 {
        self.b[1]
    }

    fn height(&self) -> i32 {
        self.b[2]
    }
}

impl<'a> PartialOrd for Building<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.height().partial_cmp(&other.height())
    }
}

impl<'a> Ord for Building<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Solution;

fn main() {}
