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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        Self::travel(root, target_sum, 1)
    }

    fn travel(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, level: usize) -> Vec<Vec<i32>> {
        if let Some(root) = root {
            let mut node = root.borrow_mut();
            let (left, right) = (node.left.take(), node.right.take());
            if left.is_none() && right.is_none() {
                if target_sum == node.val {
                    vec![vec![node.val; level]]
                } else {
                    vec![]
                }
            } else {
                let mut r1 = Self::travel(left, target_sum - node.val, level + 1);
                let mut r2 = Self::travel(right, target_sum - node.val, level + 1);
                r1.append(&mut r2);
                r1.iter_mut().for_each(|xs| xs[level - 1] = node.val);
                r1
            }
        } else {
            vec![]
        }
    }
}

impl Solution2 {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if let Some(root) = root {
            let mut node = root.borrow_mut();
            let (left, right) = (node.left.take(), node.right.take());
            if left.is_none() && right.is_none() {
                if target_sum == node.val {
                    vec![vec![node.val]]
                } else {
                    vec![]
                }
            } else {
                let r1 = Self::path_sum(left, target_sum - node.val);
                let r2 = Self::path_sum(right, target_sum - node.val);
                r1.into_iter()
                    .chain(r2.into_iter())
                    .map(|xs| vec![node.val].into_iter().chain(xs.into_iter()).collect())
                    .collect()
            }
        } else {
            vec![]
        }
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

struct Solution;
struct Solution2;

fn main() {}
