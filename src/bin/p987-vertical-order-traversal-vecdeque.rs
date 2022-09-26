use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // track column and row in our queue
        // for each node we pop, push the children to the queue, and also push this node to (col, row, val)
        let mut queue = VecDeque::<(i32, i32, Option<Rc<RefCell<TreeNode>>>)>::new();
        let mut data = Vec::<(i32, i32, i32)>::new();
        queue.push_back((0, 0, root));
        
        while let Some((col, row, node)) = queue.pop_front() {
            let mut node = match &node {
                Some(n) => n,
                None => continue,
            }.borrow_mut();
            data.push((col, row, node.val));
            queue.push_back((col - 1, row + 1, node.left.take()));
            queue.push_back((col + 1, row + 1, node.right.take()));
        }
        
        data.sort_unstable();
        let mut ans = Vec::new();
        let mut it = data.into_iter().peekable();

        while let Some((curr_col, _, val)) = it.next() {
            let mut curr = vec![val];
            while let Some((_, _, val)) = it.next_if(|(col, ..)| *col == curr_col) {
                curr.push(val);
            }
            ans.push(curr);
        }

        ans
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
