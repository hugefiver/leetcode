impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        matrix
            .windows(2)
            .map(|xss| {
                xss[0]
                    .iter()
                    .zip(xss[1].iter().skip(1))
                    .all(|(x, y)| x == y)
            })
            .all(|x| x)
    }
}

struct Solution;

fn main() {}
