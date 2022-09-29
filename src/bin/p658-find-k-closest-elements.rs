use std::cmp::Ordering;
use std::collections::VecDeque;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut lo, mut hi) = match arr.binary_search(&x) {
            Ok(i) => (i, i + 1),
            Err(i) => (i, i),
        };
        let k = k as usize;
        while hi - lo < k {
            if lo > 0 && (hi == arr.len() || x - arr[lo - 1] <= arr[hi] - x) {
                lo -= 1;
            } else {
                hi += 1;
            }
        }
        arr[lo..hi].to_vec()
    }

    pub fn find_closest_elements_2(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut ret: VecDeque<i32> = VecDeque::with_capacity(10);

        // let idx = arr.binary_search(&x).unwrap_or_else(|x| x);
        let (mut l, mut r) = match arr.binary_search(&x) {
            Ok(idx) => {
                ret.push_back(x);
                (idx.wrapping_sub(1), idx + 1)
            }
            Err(idx) => (idx.wrapping_sub(1), idx),
        };

        while ret.len() < (k as usize) {
            // println!("{} : {} : {:?}", l, r, ret);
            match (l, r) {
                (usize::MAX, _) => {
                    ret.push_back(arr[r]);
                    r = r + 1;
                }
                (_, n) if (n == arr.len()) => {
                    ret.push_front(arr[l]);
                    l = l.wrapping_sub(1);
                }
                (..) => match (x - arr[l]).cmp(&(arr[r] - x)) {
                    Ordering::Greater => {
                        ret.push_back(arr[r]);
                        r = r + 1;
                    }
                    Ordering::Less | Ordering::Equal => {
                        ret.push_front(arr[l]);
                        l = l.wrapping_sub(1);
                    }
                },
            };
        }
        ret.into_iter().collect::<Vec<i32>>()
    }
}

struct Solution;

fn main() {}
