impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        Self::foo(&candidates, target)
    }

    fn foo(arr: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        for (i, &n) in arr.iter().enumerate() {
            if n == target {
                ret.push(vec![n]);
                break;
            } else if n < target {
                let mut rr = Self::foo(&arr[i..], target - n);
                if !rr.is_empty() {
                    rr.iter_mut().for_each(|x| x.push(n));
                    ret.append(&mut rr);
                }
            } else {
                break;
            }
        }
        ret
    }
}

struct Solution;

fn main() {
    let arr = vec![2, 3, 6, 7];
    let target = 7;
    let result = Solution::combination_sum(arr, target);
    println!("{:?}", result);
}
