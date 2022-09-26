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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::travel(root, 0)
    }

    fn travel(root: Option<Rc<RefCell<TreeNode>>>, mut bs: i16) -> i32 {
        if let Some(root) = root {
            let mut node = root.borrow_mut();
            Self::bs_set(&mut bs, node.val);

            if node.left.is_none() && node.right.is_none() {
                // let bs_count = Self::bs_count(bs);
                // if bs_count == 0 || bs_count == 1 {
                //     1
                // } else {
                //     0
                // }
                if bs & (bs - 1) == 0 {
                    1
                } else {
                    0
                }
            } else {
                Self::travel(node.left.take(), bs) + Self::travel(node.right.take(), bs)
            }
        } else {
            0
        }
    }

    #[inline]
    fn bs_set(bs: &mut i16, k: i32) {
        assert!(k < 16);
        *bs ^= 1 << k;
    }

    #[inline]
    fn bs_count(bs: i16) -> i32 {
        // let mut count = 0;
        // for i in 0..16 {
        //     if bs & 1 << i != 0 {
        //         count += 1;
        //     }
        // }
        // count
        bs.count_ones() as i32
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
