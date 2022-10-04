impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let full_set: HashSet<i8> = (1..=9).collect();
        let mut col_sets = vec![full_set.clone(); 9];
        let mut row_sets = vec![full_set.clone(); 9];
        let mut box_sets = vec![vec![full_set.clone(); 3]; 3];

        board.into_iter().enumerate().map(|(ridx, row)| {
            row.into_iter().enumerate().map(|(cidx, ch)| {
                if matches!(ch, '1'..='9') {
                    let a = &mut col_sets[cidx];
                    let b = &mut row_sets[ridx];
                    let c = &mut box_sets[ridx/3][cidx/3];
                    let val = ch.to_digit(10).unwrap() as i8;
                    a.remove(&val) && b.remove(&val) && c.remove(&val)
                } else {
                    true
                }
            }).all(|x| x)
        }).all(|x| x)
    }
}

struct Solution;

fn main() {}