impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut v = vec![-1; cols];
        for i in 0..cols {
            let mut stuck = false;
            let mut y = i;
            for j in 0..rows {
                let slant = grid[j][y];
                if slant == 1 {
                    if y == cols - 1 || grid[j][y + 1] == -1 {
                        stuck = true;
                        break;
                    } else {
                        y += 1;
                    }
                } else {
                    if y == 0 || grid[j][y - 1] == 1 {
                        stuck = true;
                        break;
                    } else {
                        y -= 1;
                    }
                }
            }
            if stuck {
                v[i] = -1;
            } else {
                v[i] = y as i32;
            }
        }
        v
    }
}
struct Solution;

fn main() {}
