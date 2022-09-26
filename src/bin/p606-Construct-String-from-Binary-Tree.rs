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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            let val = node.borrow().val;
            let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
            match (&left, &right) {
                (None, None) => val.to_string(),
                (Some(_), Some(_)) => format!(
                    "{}({})({})",
                    val,
                    Self::tree2str(left),
                    Self::tree2str(right)
                ),
                (Some(_), None) => format!("{}({})", val, Self::tree2str(left)),
                (None, Some(_)) => format!("{}()({})", val, Self::tree2str(right)),
            }
        } else {
            "".to_string()
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

#[test]
fn test() {}

struct Solution {}

fn main() {}
