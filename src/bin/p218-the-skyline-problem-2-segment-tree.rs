use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut s = HashSet::new();
        for b in &buildings {
            s.insert(b[0]);
            s.insert(b[1] - 1);
            s.insert(b[1]);
        }

        let mut v = s.iter().copied().into_iter().collect::<Vec<i32>>();
        let mut mp = HashMap::new();

        v.sort();
        for a in &v {
            let i = mp.len();
            mp.insert(a, i);
        }

        let n = v.len();
        let mut tree = vec![(0, 0); 4 * n];
        let mut s = HashSet::new();

        for b in buildings {
            s.insert(b[0]);
            s.insert(b[1]);
            if let Some(left) = mp.get(&b[0]) {
                if let Some(right) = mp.get(&(b[1] - 1)) {
                    Self::update(1, 0, n - 1, *left, *right, b[2], &mut tree);
                }
            }
        }

        let mut v = s.iter().copied().into_iter().collect::<Vec<i32>>();
        v.sort();

        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut last_height = 0;
        for a in v {
            if let Some(i) = mp.get(&a) {
                let height = Self::find(1, 0, n - 1, *i, &mut tree);
                if height == last_height {
                    continue;
                }

                ret.push(vec![a, height]);
                last_height = height;
            }
        }

        ret
    }

    fn update(
        u: usize,
        left: usize,
        right: usize,
        l: usize,
        r: usize,
        val: i32,
        tree: &mut Vec<(i32, i32)>,
    ) {
        if left >= l && right <= r {
            tree[u].1 = tree[u].1.max(val);
            if left == right {
                tree[u].0 = tree[u].0.max(val);
            }
            return;
        }
        if left > r || right < l {
            return;
        }

        let mid = left + (right - left) / 2;
        Self::update(2 * u, left, mid, l, r, val, tree);
        Self::update(2 * u + 1, mid + 1, right, l, r, val, tree);
    }

    fn find(u: usize, left: usize, right: usize, i: usize, tree: &mut Vec<(i32, i32)>) -> i32 {
        if left == right {
            if tree[u].1 > tree[u].0 {
                tree[u].0 = tree[u].1
            }
            return tree[u].0;
        }

        let mid = left + (right - left) / 2;

        tree[2 * u].1 = tree[2 * u].1.max(tree[u].1);
        tree[2 * u + 1].1 = tree[2 * u + 1].1.max(tree[u].1);

        if mid >= i {
            return Self::find(2 * u, left, mid, i, tree);
        }
        Self::find(2 * u + 1, mid + 1, right, i, tree)
    }
}

struct Solution;

fn main() {}
