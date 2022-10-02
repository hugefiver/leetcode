impl Solution {
    fn next_permutation(nums: &mut [i32]) {
        if let Some(i) = (1..nums.len()).rev().find(|&i| nums[i - 1] < nums[i]) {
            let j = (i..nums.len())
                .rev()
                .find(|&j| nums[i - 1] < nums[j])
                .unwrap();
            nums.swap(i - 1, j);
            nums[i..].reverse();
        } else {
            nums.reverse();
        };
    }

    pub fn next_permutation_2(nums: &mut [i32]) {
        if let Some(i) = nums.windows(2).rposition(|w| w[0] < w[1]) {
            let (x, xs) = nums[i..].split_first_mut().unwrap();
            xs.reverse();
            let j = xs
                .binary_search_by(|a| a.cmp(x).then(std::cmp::Ordering::Less))
                .unwrap_err();
            nums.swap(i, i + j + 1);
        } else {
            nums.reverse();
        }
    }
}

struct Solution;

fn main() {}

/* C++

void nextPermutation(vector<int>& nums) {
    int n = nums.size(), k, l;
    for (k = n - 2; k >= 0; k--) {
        if (nums[k] < nums[k + 1]) {
            break;
        }
    }
    if (k < 0) {
        reverse(nums.begin(), nums.end());
    } else {
        for (l = n - 1; l > k; l--) {
            if (nums[l] > nums[k]) {
                break;
            }
        } 
        swap(nums[k], nums[l]);
        reverse(nums.begin() + k + 1, nums.end());
    }
}

*/
