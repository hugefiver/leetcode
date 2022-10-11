fn valid_solution(sudoku: &[[u8;9]; 9]) -> bool {
    use std::collections::HashSet;

    let full_set: HashSet<u8> = (1..=9).collect();
    let mut col_sets = vec![full_set.clone(); 9];
    let mut row_sets = vec![full_set.clone(); 9];
    let mut box_sets = vec![vec![full_set.clone(); 3]; 3];

    sudoku.into_iter().enumerate().map(|(ridx, row)| {
        row.into_iter().enumerate().map(|(cidx, &n)| {
            if matches!(n, 1..=9) {
                let a = &mut col_sets[cidx];
                let b = &mut row_sets[ridx];
                let c = &mut box_sets[ridx/3][cidx/3];
                
                a.remove(&n) && b.remove(&n) && c.remove(&n)
            } else {
                false
            }
        }).all(|x| x)
    }).all(|x| x)
}

#[cfg(test)]
mod sample_tests {
    use super::valid_solution;

    #[test]
    fn valid_sudoku() {
        let puzzle = [[7, 6, 9, 5, 3, 8, 1, 2, 4],
                      [2, 4, 3, 7, 1, 9, 6, 5, 8],
                      [8, 5, 1, 4, 6, 2, 9, 7, 3],
                      [4, 8, 6, 9, 7, 5, 3, 1, 2],
                      [5, 3, 7, 6, 2, 1, 4, 8, 9],
                      [1, 9, 2, 8, 4, 3, 7, 6, 5],
                      [6, 1, 8, 3, 5, 4, 2, 9, 7],
                      [9, 7, 4, 2, 8, 6, 5, 3, 1],
                      [3, 2, 5, 1, 9, 7, 8, 4, 6]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, true, "\nYour result (left) did not match expected result (right).");
    }
    
    #[test]
    fn invalid_sudoku() {
        let puzzle = [[7, 6, 9, 5, 3, 8, 1, 2, 4],
                      [2, 4, 3, 7, 1, 9, 6, 5, 8],
                      [8, 5, 1, 4, 6, 2, 9, 7, 3],
                      [4, 8, 6, 9, 7, 5, 3, 1, 2],
                      [5, 3, 7, 6, 2, 1, 4, 8, 9],
                      [1, 9, 2, 8, 4, 3, 7, 6, 5],
                      [6, 1, 8, 3, 5, 4, 2, 9, 7],
                      [9, 7, 4, 2, 8, 6, 5, 3, 1],
                      [3, 2, 5, 1, 9, 7, 8, 4, 9]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
    }
    
    #[test]
    fn invalid_with_zeroes() {
        let puzzle = [[3, 1, 5, 8, 4, 7, 6, 2, 9],
                      [4, 7, 8, 2, 9, 6, 3, 5, 0],
                      [2, 9, 6, 3, 5, 1, 7, 8, 4],
                      [7, 4, 2, 9, 6, 8, 5, 1, 3],
                      [6, 8, 9, 5, 1, 3, 4, 7, 2],
                      [5, 0, 1, 4, 7, 2, 8, 9, 6],
                      [1, 2, 4, 6, 8, 5, 9, 3, 7],
                      [8, 6, 3, 7, 2, 9, 0, 4, 5],
                      [9, 5, 7, 1, 3, 4, 2, 6, 8]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
    }
}

