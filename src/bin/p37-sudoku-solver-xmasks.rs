impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows: [u16; 9] = [0; 9];
        let mut cols: [u16; 9] = [0; 9];
        let mut boxs: [u16; 9] = [0; 9];
        for r in 0..9 {
            for c in 0..9 {
                match board[r][c] {
                    '.' => {}
                    x => {
                        let mask = 1 << ((x as u8) - ('0' as u8));
                        rows[r] |= mask;
                        cols[c] |= mask;
                        boxs[r / 3 * 3 + c / 3] |= mask;
                    }
                }
            }
        }
        Self::solve(board, &mut rows, &mut cols, &mut boxs, 0);
    }

    fn solve(
        board: &mut Vec<Vec<char>>,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxs: &mut [u16; 9],
        i: usize,
    ) -> bool {
        if i == 81 {
            return true;
        }
        let (r, c) = (i / 9, i % 9);
        if board[r][c] != '.' {
            return Self::solve(board, rows, cols, boxs, i + 1);
        }
        let b = (r / 3) * 3 + (c / 3);
        let mask = rows[r] | cols[c] | boxs[b];
        for x in 1..=9 {
            let xmask = 1 << x;
            if mask & xmask != 0 {
                continue;
            }
            rows[r] |= xmask;
            cols[c] |= xmask;
            boxs[b] |= xmask;
            board[r][c] = std::char::from_digit(x, 10).unwrap();
            if Self::solve(board, rows, cols, boxs, i + 1) {
                return true;
            }
            rows[r] ^= xmask;
            cols[c] ^= xmask;
            boxs[b] ^= xmask;
            board[r][c] = '.';
        }
        false
    }
}

struct Solution;

fn main() {
    let mut v = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(&mut v);
}
