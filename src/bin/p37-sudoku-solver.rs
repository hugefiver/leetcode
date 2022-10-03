/**
 * Something goes wrong
 * 
 */

use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Sub;
use std::rc::Rc;

struct Guess {
    pub curr: Option<i8>,
    pub set: Option<Vec<i8>>,
    pub idx: usize,
}

impl Guess {
    pub fn new() -> Self {
        Guess {
            curr: None,
            set: None,
            idx: 0,
        }
    }

    pub fn next(&mut self) -> Option<i8> {
        if self.idx < self.set.as_ref().unwrap().len() {
            let ret = Some(self.set.as_ref().unwrap()[self.idx]);
            self.idx += 1;
            self.curr = ret;
            ret
        } else {
            self.idx = 0;
            self.curr = None;
            None
        }
    }
}

enum Cell {
    Inner(i8),
    G(Guess),
}

struct Plan {
    col_sets: Vec<HashSet<i8>>,
    row_sets: Vec<HashSet<i8>>,
    cell_sets: Vec<Vec<HashSet<i8>>>,

    pan: Vec<Vec<Rc<RefCell<Cell>>>>,
    plan: Vec<(usize, usize)>,
}

impl Plan {
    fn new(
        col_sets: Vec<HashSet<i8>>,
        row_sets: Vec<HashSet<i8>>,
        cell_sets: Vec<Vec<HashSet<i8>>>,
        pan: Vec<Vec<Rc<RefCell<Cell>>>>,
        plan: Vec<(usize, usize)>,
    ) -> Self {
        assert_eq!(col_sets.len(), 9);
        assert_eq!(row_sets.len(), 9);
        assert_eq!(cell_sets.len(), 3);

        Self {
            col_sets,
            row_sets,
            cell_sets,
            pan,
            plan,
        }
    }

    fn deal(&mut self) -> bool {
        self.foo(&self.plan.clone())
    }

    fn foo(&mut self, plan: &[(usize, usize)]) -> bool {
        if plan.len() == 0 {
            return true;
        }

        let (ridx, cidx) = plan[0];
        let rest = &plan[1..];

        if let Cell::G(ref mut g) = *self.pan[ridx][cidx].clone().borrow_mut() {
            while let Some(val) = g.next() {
                if [
                    &self.cell_sets[ridx / 3][cidx / 3],
                    &self.row_sets[ridx],
                    &self.col_sets[cidx],
                ]
                .iter()
                .all(|s| s.contains(&val))
                {
                    self.cell_sets[ridx / 3][cidx / 3].remove(&val);
                    self.row_sets[ridx].remove(&val);
                    self.col_sets[cidx].remove(&val);

                    let ok = self.foo(rest);
                    if ok {
                        return true;
                    } else {
                        self.cell_sets[ridx / 3][cidx / 3].insert(val);
                        self.row_sets[ridx].insert(val);
                        self.col_sets[cidx].insert(val);
                    }
                }
            }
            false
        } else {
            unreachable!()
        }
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        use Cell::*;
        let full_set: HashSet<i8> = (1..=9).collect();
        let mut col_sets = vec![full_set.clone(); 9];
        let mut row_sets = vec![full_set.clone(); 9];
        let mut cell_sets = vec![vec![full_set.clone(); 3]; 3];

        let mut plans = Vec::<(usize, usize)>::new();

        let pan: Vec<Vec<Rc<RefCell<Cell>>>> = board
            .into_iter()
            .enumerate()
            .map(|(ridx, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(cidx, ch)| match ch {
                        '1'..='9' => {
                            let n = ch.to_digit(10).unwrap() as i8;
                            row_sets[ridx].remove(&n);
                            col_sets[cidx].remove(&n);
                            cell_sets[ridx / 3][cidx / 3].remove(&n);
                            Rc::new(RefCell::new(Inner(n)))
                        }
                        '.' => {
                            plans.push((ridx, cidx));
                            Rc::new(RefCell::new(G(Guess::new())))
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        plans.iter().cloned().for_each(|(ridx, cidx)| {
            if let G(ref mut g) = *pan[ridx][cidx].borrow_mut() {
                g.set = Some(
                    full_set
                        .sub(&row_sets[ridx])
                        .sub(&col_sets[cidx])
                        .sub(&cell_sets[ridx / 3][cidx / 3])
                        .into_iter()
                        .collect(),
                );
            } else {
                unreachable!()
            }
        });

        let mut plan = Plan::new(col_sets, row_sets, cell_sets, pan, plans);
        let ok = plan.deal();
        if !ok {
            panic!("failed")
        } else {
            *board = plan
                .pan
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|c| {
                            let cell = &*c.borrow();
                            match cell {
                                Inner(x) => char::from_digit(*x as u32, 10).unwrap(),
                                G(g) => char::from_digit(g.curr.unwrap() as u32, 10).unwrap(),
                            }
                        })
                        .collect()
                })
                .collect();
        }
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
