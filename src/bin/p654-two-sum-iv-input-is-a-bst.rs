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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        Self::foo(root, k, &mut HashSet::new())
    }

    pub fn foo(root: Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>) -> bool {
        if let Some(root) = root {
            let mut node = root.borrow_mut();
            if set.contains(&(k - node.val)) {
                return true;
            } else {
                set.insert(node.val);
                Self::foo(node.left.take(), k, set) || Self::foo(node.right.take(), k, set)
            }
        } else {
            false
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

fn main() {}
