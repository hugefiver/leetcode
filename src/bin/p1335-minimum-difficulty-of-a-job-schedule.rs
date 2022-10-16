impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        if job_difficulty.len() < d as usize || d == 0 {
            return -1;
        }
        Self::foo(job_difficulty, d)
    }

    fn foo(jobs: Vec<i32>, d: i32) -> i32 {
        let mut dp = vec![vec![-1; jobs.len()]; d as usize];
        dp[0] = jobs.iter().fold(Vec::new(), |mut acc, &x| {
            if let Some(&last) = acc.last() {
                acc.push(x.max(last));
            } else {
                acc.push(x);
            }
            acc
        });
        for i in 1..d as usize {
            for j in i..jobs.len() {
                if i == j {
                    dp[i][j] = dp[i - 1][j - 1] + jobs[j];
                } else {
                    dp[i][j] = (i..=j)
                        .map(|idx| {
                            dp[i - 1][idx - 1] + jobs[idx..=j].iter().max().unwrap_or(&0)
                        })
                        .min()
                        .unwrap();
                }
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}

impl Solution1 {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        Self::foo(&job_difficulty, d)
    }

    fn foo(jobs: &[i32], d: i32) -> i32 {
        if jobs.len() < d as usize {
            -1
        } else if d == 1 {
            jobs.into_iter().max().cloned().unwrap()
        } else {
            let mut min = i32::MAX;
            for idx in 1..=(jobs.len() - d as usize + 1) {
                let t1 = jobs[..idx].iter().max().cloned().unwrap();
                let t2 = Self::foo(&jobs[idx..], d - 1);
                if t2 == -1 {
                    break;
                }
                if t1 + t2 < min {
                    min = t1 + t2;
                }
            }
            if min < i32::MAX {
                min
            } else {
                -1
            }
        }
    }
}

struct Solution;
struct Solution1;

fn main() {
    for (j, i) in [
        (vec![6, 5, 4, 3, 2, 1], 2),
        (vec![9, 9, 9], 4),
        (vec![1, 1, 1], 3),
        (
            vec![
                27, 54, 1, 95, 78, 14, 85, 11, 87, 6, 69, 34, 97, 27, 65, 56, 2, 28, 39, 74, 37,
                12, 11, 68, 16, 23, 74, 16, 7, 12, 30, 15, 80, 55, 83, 61, 30, 17, 63, 91, 7, 32,
                31, 10, 100, 85, 26, 78, 48, 22, 56, 24, 4, 26, 5, 100, 20, 11, 97, 21, 23, 77, 88,
                80, 62, 64, 93, 34, 8, 51, 57, 29, 67, 31, 77, 16, 55, 31, 49, 76, 78, 5, 90, 27,
                3, 56, 62, 51, 33, 26, 51, 8, 54, 53, 12, 83, 60, 60, 98, 95, 24, 57, 61, 10, 18,
                93, 73, 34, 42, 89, 72, 93, 38, 13, 76, 43, 33, 31, 100, 60, 48, 34, 16, 44, 91,
                22, 23, 33, 10, 57, 30, 64, 19, 15, 100, 37, 83, 93, 96, 66, 19, 28, 61, 3, 55, 83,
                60, 28, 36, 83, 99, 35, 59, 73, 40, 27, 34, 84, 20, 75, 94, 90, 95, 30, 14, 46, 24,
                57, 97, 38, 57, 39, 38, 53, 20, 26, 63, 92, 90, 76, 68, 25, 47, 61, 69, 19, 9, 14,
                93, 37, 32, 98, 59, 37, 98, 64, 100, 99, 93, 15, 91, 82, 18, 4, 67, 3, 1, 79, 36,
                70, 64, 85, 18, 28, 21, 43, 79, 30, 74, 98, 42, 16, 1, 72, 33, 52, 20, 23, 21, 75,
                93, 18, 32, 42, 2, 95, 60, 64, 71, 1, 60, 13, 42, 3, 55, 61, 69, 71, 34, 42, 75,
                29, 71, 80, 65, 87, 9, 29, 78, 85, 84, 93, 35, 1, 62, 73, 39, 66, 45, 2, 41, 93,
                69, 26, 60, 18, 15, 5, 91, 38, 100, 83, 59, 26, 77, 23, 89, 79, 90, 38, 56, 54, 86,
                16, 76, 85, 52, 71, 5, 78,
            ],
            10,
        ),
    ] {
        assert_eq!(
            Solution::min_difficulty(j.clone(), i),
            Solution1::min_difficulty(j, i)
        );
    }
}
