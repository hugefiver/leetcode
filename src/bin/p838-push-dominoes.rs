impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut doms: Vec<_> = dominoes.chars().collect();
        let trades: Vec<_> = doms
            .iter()
            .cloned()
            .enumerate()
            .skip(1)
            .filter(|&(_, fall)| fall == 'L' || fall == 'R')
            .collect();
        let mut curr_idx = 0usize;
        let mut trade_iter = trades.into_iter();
        while curr_idx < doms.len() {
            let curr_trade = doms[curr_idx];
            if let Some(next) = trade_iter.next() {
                let (idx, trade) = next;
                if curr_idx >= idx {
                    continue;
                }
                match (curr_trade, trade) {
                    ('R', 'L') => {
                        let inmid = idx - curr_idx - 1;
                        let x = curr_idx + (inmid / 2) + 1;
                        let (left, right) = if inmid % 2 == 0 {
                            (x, x)
                        } else {
                            (x, x+1)
                        };
                        doms[curr_idx+1..left].fill('R');
                        doms[right..idx].fill('L');
                        curr_idx = idx + 1;
                    }
                    (_, 'L') => {
                        doms[curr_idx..idx].fill('L');
                        curr_idx = idx + 1;
                    },
                    ('R', 'R') => {
                        doms[curr_idx+1..idx].fill('R');
                        curr_idx = idx;
                    } 
                    _ => {
                        curr_idx = idx;
                    }
                }
            } else {
                if curr_trade == 'R' {
                    doms[curr_idx+1..].fill('R');
                }
                break;
            }
        }

        doms.into_iter().collect()
    }
}

struct Solution;

fn main() {}
