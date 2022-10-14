impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            None
        } else {
            Self::travel(&mut head, 0);
            head
        }
        
    }

    fn travel(head: &mut Option<Box<ListNode>>, n: i32) -> i32 {
        if let Some(root) = head {
            let r = Self::travel(&mut root.next, n+1);
            if r == 0 {
                root.next = root.next.as_mut().unwrap().next.take();
                -1
            } else {
                r - 1
            }
        } else {
            (n + 1) / 2
        }
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

struct Solution;

fn main() {}
