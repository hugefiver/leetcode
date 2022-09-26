use std::{rc::Rc, borrow::BorrowMut};

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::foo(head)
    }

    fn foo(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(ref mut car) = head {
            if let Some(mut cdr) = car.next.take() {
                let next = cdr.next.take();
                car.next = Self::foo(next);
                cdr.next = head;
                return Some(cdr);
            }
        }
        head
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
