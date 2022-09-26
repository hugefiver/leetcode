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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();

        Self::travel(root, &mut map, 0, 0);
        let mut kvs: Vec<(i32, Vec<(i32, i32)>)> = map.into_iter().collect();
        kvs.sort_by_key(|&(x, _)| x);
        kvs.iter_mut().map(|(_, vs)| -> Vec<i32> {
            vs.sort_by_key(|&v| v);
            vs.iter().map(|&(_, vs)| vs).collect()
        }).collect()
    }

    fn travel(
        root: Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, Vec<(i32, i32)>>,
        level: i32,
        order: i32,
    ) {
        if let Some(root) = root {
            let node = root.borrow();
            if let Some(v) = map.get_mut(&order) {
                v.push((level, node.val));
            } else {
                map.insert(order, vec![(level, node.val)]);
            }
            Self::travel(node.left.clone(), map, level + 1, order - 1);
            Self::travel(node.right.clone(), map, level + 1, order + 1);
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
