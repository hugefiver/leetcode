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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        return Self::foo(head, &mut n.clone());
    }

    fn foo(head: Option<Box<ListNode>>, n: &mut i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let m = *n;
            let next = Self::foo(node.next.take(), n);
            *n += 1;
            if *n == m {
                next
            } else {
                node.next = next;
                Some(node)
            }
        } else {
            *n = 0;
            head
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}
