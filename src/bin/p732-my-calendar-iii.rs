use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarThree {
    tree: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.tree.entry(start).or_insert(0) += 1;
        *self.tree.entry(end).or_insert(0) -= 1;
        let mut active = 0;
        let mut ans = 0;
        for &ct in self.tree.values() {
            active += ct;
            ans = ans.max(active);
        }
        ans
    }
}

// use std::{
//     collections::{BTreeMap, HashSet},
//     ops::Bound,
// };

// #[derive(Default)]
// struct MyCalendarThree {
//     starts: BTreeMap<i32, HashSet<usize>>,
//     ends: BTreeMap<i32, HashSet<usize>>,
//     items: Vec<(i32, i32)>,
//     curr: usize,
// }

// fn map_insert_set<K, T>(map: &mut BTreeMap<K, HashSet<T>>, key: K, val: T)
// where
//     K: std::cmp::Ord,
//     T: Eq + std::hash::Hash,
// {
//     // if let Some(s) = map.get_mut(&key) {
//     //     s.insert(val);
//     // } else {
//     //     map.insert(key, [val].into());
//     // }
//     map.entry(key).or_insert(Default::default()).insert(val);
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl MyCalendarThree {
//     fn new() -> Self {
//         Default::default()
//     }

//     fn book(&mut self, start: i32, end: i32) -> i32 {
//         let a: HashSet<_> = self
//             .starts
//             .range(..end)
//             .fold(HashSet::new(), |mut acc, (_, s)| {
//                 acc.extend(s.iter().cloned());
//                 acc
//             });
//         let b: HashSet<_> = self
//             .ends
//             .range(start..)
//             .fold(HashSet::new(), |mut acc, (_, s)| {
//                 acc.extend(s.iter().cloned());
//                 acc
//             });

//         let mut included = false;
//         let s: Vec<_> = a.intersection(&b).cloned().collect();
//         for &i in s.iter() {
//             let (x, y) = self.items[i];
//             if x <= start && end <= y {
//                 included = true;
//                 break;
//             }
//         }

//         let ret = if !included {
//             map_insert_set(&mut self.starts, start, self.curr);
//             map_insert_set(&mut self.ends, end, self.curr);
//             s.len() as i32 + 1
//         } else {
//             s.len() as i32
//         };

//         self.items.push((start, end));
//         self.curr += 1;

//         ret
//     }
// }

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */

fn main() {
    let mut c = MyCalendarThree::new();
    for (x, y) in [(10, 20), (50, 60), (10, 40), (5, 15), (5, 10), (25, 55)] {
        println!("[{}, {}) => {}", x, y, c.book(x, y));
    }
}
