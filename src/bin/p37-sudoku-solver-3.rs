struct Solution {}
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut left_row = vec![vec![0; 10]; 10];
        let mut left_col = vec![vec![0; 10]; 10];
        let mut left_box = vec![vec![0; 10]; 10];
        for i in 0..9 {
            for j in 0..9 {
                if *board.get(i).unwrap().get(j).unwrap() != '.' {
                    let k = board.get(i).unwrap().get(j).unwrap().to_digit(10).unwrap()
                        - '0'.to_digit(10).unwrap();
                    left_row[i][k as usize] = 1;
                    left_col[j][k as usize] = 1;
                    left_box[i / 3 * 3 + j / 3][k as usize] = 1;
                }
            }
        }
        let mut result = false;
        Self::solve(
            board,
            &mut left_row,
            &mut left_col,
            &mut left_box,
            0,
            0,
            &mut result,
        );
        println!("{}, {:?}", result, board);
    }
    pub fn solve(
        board: &mut Vec<Vec<char>>,
        left_row: &mut Vec<Vec<u32>>,
        left_col: &mut Vec<Vec<u32>>,
        left_box: &mut Vec<Vec<u32>>,
        i: usize,
        j: usize,
        result: &mut bool,
    ) {
        if i == 9 {
            *result = true;
            println!("{:?}", board);
            return;
        }
        if *result {
            return;
        }
        if *board.get(i).unwrap().get(j).unwrap() == '.' {
            let tmp = board[i][j];
            for k in 1..10 {
                if left_row[i][k] == 1 || left_col[j][k] == 1 || left_box[i / 3 * 3 + j / 3][k] == 1
                {
                    continue;
                }
                left_row[i][k as usize] = 1;
                left_col[j][k as usize] = 1;
                left_box[i / 3 * 3 + j / 3][k as usize] = 1;

                board[i][j] = std::char::from_digit(k as u32, 10).unwrap();

                if j == 8 {
                    Self::solve(board, left_row, left_col, left_box, i + 1, 0, result);
                } else {
                    Self::solve(board, left_row, left_col, left_box, i, j + 1, result);
                }
                left_row[i][k] = 0;
                left_col[j][k] = 0;
                left_box[i / 3 * 3 + j / 3][k] = 0;
            }
            if *result {
                return;
            }
            board[i][j] = tmp;
        } else {
            if j == 8 {
                Self::solve(board, left_row, left_col, left_box, i + 1, 0, result);
            } else {
                Self::solve(board, left_row, left_col, left_box, i, j + 1, result);
            }
        }
    }
}
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
