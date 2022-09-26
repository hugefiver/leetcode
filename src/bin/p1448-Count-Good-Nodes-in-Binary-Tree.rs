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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let max = node.borrow().val;
            Self::travel(Some(node), max)
        } else {
            0
        }
    }

    fn travel(root: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        if let Some(root) = root {
            let mut max = max;
            let mut goods = 0;
            if root.borrow().val >= max {
                max = root.borrow().val;
                goods += 1;
            }
            goods += Self::travel(root.borrow_mut().left.take(), max);
            goods += Self::travel(root.borrow_mut().right.take(), max);
            goods
        } else {
            0
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
      right: None
    }
  }
}

#[test]
fn test() {}

struct Solution {}

fn main() {}
