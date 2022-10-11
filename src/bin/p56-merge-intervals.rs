impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        intervals.sort_unstable();
        let mut ret = vec![];
        ret.push(intervals[0].clone());
        for r in intervals[1..].into_iter() {
            let x = ret.last().unwrap()[1];
            if r[0] <= x {
                ret.last_mut().unwrap()[1] = r[1].max(x);
            } else {
                ret.push(r.to_owned());
            }
        }
        ret
    }
}

struct Solution;

fn main() {}
