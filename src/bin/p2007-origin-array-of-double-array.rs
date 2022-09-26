impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        if changed.len() < 2 {
            return vec![];
        }

        let mut input = changed;
        input.sort_unstable();
        let mut arr: Vec<_> = input.into_iter().map(|x| Some(x)).collect();
        let mut ret = vec![];

        let (mut i, mut k) = (0, 1);
        while k < arr.len() {
            if i == k {
                k += 1;
                continue;
            }
            if let (Some(x), Some(y)) = (arr[i], arr[k]) {
                if x * 2 == y {
                    ret.push(x);
                    arr[i] = None;
                    arr[k] = None;
                    i += 1;
                }
                k += 1
            } else if arr[k].is_none() {
                k += 1;
            } else {
                i += 1;
            }
        }

        let ok = arr[i..].into_iter().all(|x| x.is_none());
        if ok {
            ret
        } else {
            vec![]
        }
    }

    pub fn find_original_array_2(changed: Vec<i32>) -> Vec<i32> {
        use std::collections::BTreeMap;
        if changed.len() < 2 {
            return vec![];
        }

        let mut map: BTreeMap<_, i32> = BTreeMap::new();
        changed.into_iter().for_each(|x| {
            if let Some(v) = map.get_mut(&x) {
                *v += 1;
            } else {
                map.insert(x, 1);
            }
        });
        let mut ret = vec![];

        for k in map.keys().cloned().collect::<Vec<_>>() {
            let t = map.get(&k);
            let v;
            if t.is_none() {
                continue;
            } else {
                v = *t.unwrap();
            };

            if k == 0 {
                if v % 2 == 0 {
                    ret.extend(vec![0; v as usize / 2]);
                    map.remove(&k);
                } else {
                    break;
                }
            } else {
                let k2 = k * 2;
                if let Some(&v2) = map.get(&k2) {
                    if v < v2 {
                        map.remove(&k);
                        map.insert(k2, v2 - v);
                        ret.extend(vec![k; v as usize]);
                    } else if v > v2 {
                        map.remove(&k2);
                        map.insert(k, v - v2);
                        ret.extend(vec![k; v2 as usize]);
                    } else {
                        map.remove(&k);
                        map.remove(&k2);
                        ret.extend(vec![k; v as usize]);
                    }
                } else {
                    break;
                }
            }
        }

        if map.is_empty() {
            ret
        } else {
            vec![]
        }
    }

    pub fn find_original_array_3(changed: Vec<i32>) -> Vec<i32> {
        use std::collections::{btree_map::Entry, BTreeMap};
        if changed.len() % 2 == 1 {
            return vec![];
        }
        // `pool` is a frequency table that is sorted by key.
        let mut pool: BTreeMap<i32, u32> = BTreeMap::new();
        for ch in changed {
            *pool.entry(ch).or_default() += 1;
        }
        let mut retval = vec![];
        while !pool.is_empty() {
            // We would use `.pop_last()`, but:
            // https://github.com/rust-lang/rust/issues/62924
            // Safety of `.unwrap()`: known to be non-empty from loop condition
            let biggest: i32 = *pool.keys().next_back().unwrap();
            if let Entry::Occupied(mut occ_en) = pool.entry(biggest) {
                if *occ_en.get() == 1 {
                    occ_en.remove_entry();
                } else {
                    *occ_en.get_mut() -= 1;
                }
            }
            if biggest % 2 == 1 {
                return vec![];
            }
            let half = biggest / 2;
            if let Entry::Occupied(mut occ_en) = pool.entry(half) {
                if *occ_en.get() == 1 {
                    occ_en.remove_entry();
                } else {
                    *occ_en.get_mut() -= 1;
                }
                retval.push(half);
            } else {
                return vec![];
            }
        }
        retval
    }
}

#[test]
fn test() {}

struct Solution {}

fn main() {
    Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]);
}
