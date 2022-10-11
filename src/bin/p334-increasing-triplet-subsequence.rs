impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut ns = Vec::new();
        for i in nums.into_iter() {
            if ns.is_empty() {
                ns.push(i);
            } else if ns.len() == 2 {
                if i > ns[1] {
                    ns.push(i);
                    return true;
                } else if i > ns[0] {
                    ns[1] = i;
                } else {
                    ns[0] = i;
                }
            } else if ns.len() == 1 {
                if i > ns[0] {
                    ns.push(i);
                } else if i <= ns[0] {
                    ns[0] = i;
                }
            }
        }
        false
    }

    fn increasing_triplet_1(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let (mut a, mut b) = (i32::MAX, i32::MAX);
        for i in nums.into_iter() {
            if i > b {
                return true;
            }
            if i > a {
                b = i.min(b);
            } else {
                a = i;
            }
        }

        false
    }
}

struct Solution;

fn main() {}
