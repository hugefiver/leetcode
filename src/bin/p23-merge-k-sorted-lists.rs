// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists: Vec<_> = lists.into_iter().filter(Option::is_some).collect();
        if lists.len() == 0 {
            None
        } else {
            let mut pre = Box::new(ListNode::new(0));
            Self::merge(&mut pre, &mut lists);
            pre.next
        }
    }

    fn merge(pre: &mut Box<ListNode>, lists: &mut Vec<Option<Box<ListNode>>>) {
        if lists.len() == 1 {
            pre.next = lists[0].take();
            return;
        }

        let (mut min_idx, mut min) = (0, i32::MAX);
        for (i, node) in lists.into_iter().enumerate() {
            let val = node.as_ref().unwrap().val;
            if val < min {
                min_idx = i;
                min = val;
            }
        }

        let mut this = lists[min_idx].take();
        let next = this.as_mut().unwrap().next.take();
        if next.is_none() {
            lists.remove(min_idx);
        } else {
            lists[min_idx] = next;
        }
        pre.next = this;
        Self::merge(&mut pre.next.as_mut().unwrap(), lists)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
