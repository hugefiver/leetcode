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
    pub fn add_one_row(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            Some(Rc::new(RefCell::new(TreeNode::node(val, root, None))))
        } else {
            if let Some(root) = root {
                let mut node = root.borrow_mut();

                let (left, right) = if depth == 2 {
                    (
                        Some(Rc::new(RefCell::new(TreeNode::node(
                            val,
                            node.left.take(),
                            None,
                        )))),
                        Some(Rc::new(RefCell::new(TreeNode::node(
                            val,
                            None,
                            node.right.take(),
                        )))),
                    )
                } else {
                    (
                        Self::add_one_row(node.left.take(), val, depth - 1),
                        Self::add_one_row(node.right.take(), val, depth - 1),
                    )
                };

                node.left = left;
                node.right = right;
                drop(node);

                Some(root)
            } else {
                None
            }
        }
    }
}

impl TreeNode {
    #[inline]
    pub fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> TreeNode {
        Self { val, left, right }
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
