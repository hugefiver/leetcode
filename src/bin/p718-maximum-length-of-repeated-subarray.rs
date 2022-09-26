impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len()]; nums1.len()];
        let mut max = 0;

        for (i, &x) in nums1.iter().enumerate() {
            for (j, &y) in nums2.iter().enumerate() {
                if x == y {
                    let value = dp
                        .get(i - 1)
                        .and_then(|v| v.get(j - 1))
                        .cloned()
                        .unwrap_or(0)
                        + 1;
                    dp[i][j] = value;
                    if value > max {
                        max = value;
                    }
                }
            }
        }
        max
    }

    pub fn find_length_2(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (i, j) in (0..nums1.len())
            .map(|i| (i, 0))
            .chain((1..nums2.len()).map(|j| (0, j)))
        {
            let mut iter1 = nums1[i..].iter();
            let mut iter2 = nums2[j..].iter();
            let mut dp = 0;
            while let (Some(&x), Some(&y)) = (iter1.next(), iter2.next()) {
                if x == y {
                    dp += 1;
                    ans = ans.max(dp);
                } else {
                    dp = 0;
                }
            }
        }
        ans
    }

    pub fn find_length_3(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums1.len() {
            let mut c = 0;
            for j in 0..nums2.len() {
                if i + j >= nums1.len() {
                    break;
                }
                if nums1[i + j] == nums2[j] {
                    c += 1;
                } else {
                    c = 0;
                }
                ans = ans.max(c);
            }
        }
        for j in 0..nums2.len() {
            let mut c = 0;
            for i in 0..nums1.len() {
                if i + j >= nums2.len() {
                    break;
                }
                if nums2[i + j] == nums1[i] {
                    c += 1;
                } else {
                    c = 0;
                }
                ans = ans.max(c);
            }
        }
        ans
    }
}

/// Wall time: 23ms.
/// Space used: 2.3MB.
/// Complexities: space O(1); time O(m(n+m)), where m is the length of the shorter input and n is the length of the longer input.
///
/// See also:
/// https://en.wikipedia.org/wiki/Convolution (discrete cross-correlation)
///
/// Idea:
/// Each input array represents a signal.
/// We "slide" the two along each other and look at all of the overlaps.
/// However, rather than computing the sum-product of each overlap, we instead compute the longest equal sequence.
///
impl Solution2 {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let longer = if n1 > n2 { &nums1[..] } else { &nums2[..] };
        let shorter = if n1 > n2 { &nums2[..] } else { &nums1[..] };
        let max_width = shorter.len();
        let mut overall_longest = 0;
        // This is convolution, but without inverting the window.
        // This is also called cross-correlation.
        // `longer` is the signal; `shorter` is the convolution function/window.
        for diag in 0..(n1 + n2 - 1) {
            // These clamping functions also act as signals; like x * u(x);, but shifted.
            let idx_short_start = shorter.len().saturating_sub(diag + 1);
            let idx_long_start = diag.saturating_sub(shorter.len() - 1);
            // This one is trapezoidal: inc; flatline; dec.
            let width = (1 + diag).min(max_width).min(n1 + n2 - 1 - diag);
            let mut local_longest = 0;
            // println!("{diag:?}\t{idx_long_start:?}\t{idx_short_start:?}\t{width:?}");
            for s in 0..width {
                // Can we keep going, or do we have to start over?
                if longer[idx_long_start + s] == shorter[idx_short_start + s] {
                    local_longest += 1;
                    overall_longest = overall_longest.max(local_longest);
                } else {
                    local_longest = 0;
                }
            }
        }
        overall_longest
    }
}

#[test]
fn test() {}

struct Solution;
struct Solution2;

fn main() {}
