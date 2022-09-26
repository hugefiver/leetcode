// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let sums = Self::travel(root, 1, vec![]);
        sums.iter()
            .map(|&(sum, count)| {
                if sum == 0 {
                    0 as f64
                } else {
                    sum as f64 / count as f64
                }
            })
            .collect()
    }

    fn travel(
        root: Option<Rc<RefCell<TreeNode>>>,
        level: usize,
        sums: Vec<(i64, u32)>,
    ) -> Vec<(i64, u32)> {
        let mut sums = sums;
        if let Some(root) = root {
            let node = root.borrow();
            let val = node.val;
            while sums.len() < level as usize {
                sums.push((0, 0));
            }
            let (sum, count) = sums[level - 1];
            sums[level-1] = (sum as i64 + val as i64, count + 1);

            sums = Self::travel(node.left.clone(), level + 1, sums);
            sums = Self::travel(node.right.clone(), level + 1, sums);
        }
        sums
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[test]
fn test() {}

fn main() {}

struct Solution {}
