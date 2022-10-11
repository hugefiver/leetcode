use itertools::Itertools;

fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    if intervals.is_empty() {
        return 0;
    }
    let intervals: Vec<_> = intervals.into_iter().cloned().sorted_unstable().collect();

    let (left, mut right) = intervals[0];
    let mut sum = right - left;
    intervals[1..].into_iter().for_each(|&(l, r)| {
        if l >= right {
            sum += r - l;
            right = r;
        } else if r >= right {
            sum += r - right;
            right = r;
        }
    });

    sum
}

#[cfg(test)]
mod sample_tests {
    use super::*;
    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
    }

    #[test]
    fn large_intervals() {
        assert_eq!(
            sum_intervals(&[(-1_000_000_000, 1_000_000_000)]),
            2_000_000_000,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]),
            100_000_030,
            "{}",
            ERR_MSG
        );
    }
}
