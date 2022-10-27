impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();

        let ones = (0..n)
            .flat_map(|y| (0..n).map(move |x| (y, x)))
            .filter(|(y, x)| img2[*y][*x] == 1)
            .collect::<Vec<_>>();

        (0..n)
            .flat_map(|y| (0..n).map(move |x| (y, x)))
            .filter(|(y, x)| img1[*y][*x] == 1)
            .flat_map(|(y1, x1)| ones.iter().copied().map(move |(y2, x2)| (y1, x1, y2, x2)))
            .fold(
                vec![vec![0; n * 2 - 1]; n * 2 - 1],
                |mut v, (y1, x1, y2, x2)| {
                    v[((n * 2 - 1) / 2) + y2 - y1][(n * 2 - 1) / 2 + x2 - x1] += 1;
                    v
                },
            )
            .into_iter()
            .flat_map(std::convert::identity)
            .max()
            .unwrap()
    }

    pub fn largest_overlap_2(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let enumerate_ones = |img: Vec<Vec<i32>>| {
            img.into_iter().enumerate().flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(j, val)| (val == 1).then(|| (i, j)))
            })
        };
        let ones2 = enumerate_ones(img2).collect::<Vec<_>>();
        let mut transform_freqs = vec![vec![0; 2 * n]; 2 * n];
        let mut ret = 0;
        for (r1, c1) in enumerate_ones(img1) {
            for (r2, c2) in ones2.iter() {
                let entry = &mut transform_freqs[n + r1 - r2][n + c1 - c2];
                *entry += 1;
                ret = ret.max(*entry);
            }
        }
        ret
    }
}

impl Solution2 {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let (mut res, n) = (0, a.len());
        let mut count = vec![vec![0; n * 2]; n * 2];
        for i in 0..n {
            for j in 0..n {
                for p in 0..n {
                    for q in 0..n {
                        if a[i][j] != 0 && b[p][q] != 0 {
                            let c = &mut count[n + i - p][n + j - q];
                            *c += 1;
                            res = std::cmp::max(res, *c);
                        }
                    }
                }
            }
        }
        res
    }

    pub fn largest_overlap_2(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let (mut res, n) = (0, a.len());
        for p in 0..n * 2 {
            for q in 0..n * 2 {
                let mut s = 0;
                for i in 0..n {
                    for j in 0..n {
                        if i + p < n || j + q < n || i + p - n >= n || j + q - n >= n {
                            continue;
                        }
                        if a[i][j] == 1 && b[i + p - n][j + q - n] == 1 {
                            s += 1
                        }
                    }
                }
                res = std::cmp::max(res, s);
            }
        }
        res
    }

    pub fn largest_overlap_3(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        fn check(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> i32 {
            let (mut res, n) = (0, a.len());
            for i in 0..n {
                for j in 0..n {
                    let mut s = 0;
                    for p in i..n {
                        for q in j..n {
                            if a[p][q] != 0 && b[p - i][q - j] != 0 {
                                s += 1;
                            }
                        }
                    }
                    res = std::cmp::max(res, s);
                }
            }
            res
        }
        std::cmp::max(check(&a, &b), check(&b, &a))
    }
}

struct Solution;
struct Solution2;

fn main() {}
