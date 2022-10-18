impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        Self::foo(&candidates, target)
    }

    fn foo(arr: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut last = None;
        for (i, &n) in arr.iter().enumerate() {
            if n == target {
                ret.push(vec![n]);
                break;
            } else if last.is_some() && n == last.unwrap() {
                continue;
            } else if n < target {
                if i == arr.len() - 1{
                    break;
                }
                let mut rr = Self::foo(&arr[i+1..], target - n);
                if !rr.is_empty() {
                    rr.iter_mut().for_each(|x| x.push(n));
                    ret.append(&mut rr);
                } 
                last = Some(n);
            } else {
                break;
            }
        }
        // ret.sort_unstable();
        // ret.dedup();
        ret
    }
}

struct Solution;

fn main() {
}
