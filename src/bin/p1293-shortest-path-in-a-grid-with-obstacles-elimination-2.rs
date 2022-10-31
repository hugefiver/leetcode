impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::VecDeque;

        const DIRS: [i32; 5] = [0, 1, 0, -1, 0];
        let m = grid.len();
        let n = grid[0].len();
        let mut q = VecDeque::<(i32, i32, i32)>::new();
        let mut visited = vec![vec![-1; n]; m];
        visited[0][0] = k;
        q.push_back((0, 0, k));
        let mut steps = 0;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let cur = q.pop_front().unwrap();
                if cur.2 < 0 {
                    continue;
                }
                if cur.0 == m as i32 - 1 && cur.1 == n as i32 - 1 {
                    return steps;
                }
                for k in 0..4 {
                    let x = cur.0 + DIRS[k];
                    let y = cur.1 + DIRS[k + 1];
                    if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                        let live = if grid[x as usize][y as usize] == 1 {
                            cur.2 - 1
                        } else {
                            cur.2
                        };
                        if live > visited[x as usize][y as usize] {
                            visited[x as usize][y as usize] = live;
                            q.push_back((x, y, live));
                        }
                    }
                }
            }
            steps += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        assert_eq!(
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0]
                ],
                1
            ),
            6
        );
    }

    #[test]
    fn test_shortest_path_02() {
        assert_eq!(
            Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1),
            -1
        )
    }
}
struct Solution;

fn main() {}
