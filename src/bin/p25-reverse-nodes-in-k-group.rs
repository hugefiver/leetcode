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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut rev_head = ListNode::new(0);

        // This guy points to the next field of the last element in rev_head
        let mut last = &mut rev_head.next;

        loop {
            let (reversed, leftover) = Self::reverse(head, k as usize);

            match (reversed, leftover) {
                // Reversed some stuff and we might have extra elements left over
                (Some(reversed), leftover) => {
                    head = leftover;
                    *last = Some(reversed);

                    for _ in 0..k {
                        match last.as_mut() {
                            Some(last_ref) => last = &mut last_ref.next,
                            None => unreachable!(),
                        }
                    }
                }
                // Nothing more has been reversed; we might have a leftover and the job is done
                (None, leftover) => {
                    *last = leftover;
                    break;
                }
            }
        }

        rev_head.next
    }

    /// # Returns
    /// `(reversed, leftover)`
    /// If there were more than `count` nodes, reversed contains the reversed nodes. Else, it's None.
    /// Leftover contains the nodes that weren't reversed (if any).
    fn reverse(
        mut head: Option<Box<ListNode>>,
        count: usize,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut rev_head = ListNode::new(0);

        // put the elements from head into target, in reversed order.
        let rev = |head: &mut Option<Box<ListNode>>, target: &mut ListNode| {
            let mut i = 0;

            while let Some(ref mut node) = head {
                let new_node = node.next.take();
                let back = target.next.take();
                node.next = back;
                target.next = head.take();
                *head = new_node;

                i += 1;

                if i == count {
                    break;
                }
            }

            i
        };

        let r = rev(&mut head, &mut rev_head);

        let mut reversed = rev_head.next;

        // We haven't reversed enough nodes, thus we need to put them back into the original order
        // And return them as leftover
        if r < count {
            let mut undo_head = ListNode::new(0);

            // Not enough nodes, we have to put them back into the same order
            rev(&mut reversed, &mut undo_head);

            (None, undo_head.next)
        } else {
            // We've reversed enough nodes, return them as reversed and head as leftover
            (reversed, head)
        }
    }
}

impl Solution2 {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        // 3 components:
        //  - head: remaining list
        //  - k_group: current accumulating group, targeting k elements
        //  - prev_tail: tail of already processed part of list
        //
        // Basic idea:
        // Take nodes from head, prepend the node to k_group, so after doing it k times,
        // k_group will be the reversed list of size k taken from head.
        // Then we append k_group to prev_tail, and search for the new prev_tail.
        // If there is not enough nodes to form a k_group of size k, that means the current k_group is the last group
        // and its size is smaller than k. In this case, we reverse the k_group again to revert the change, and append
        // it to prev_tail, then return.
        let mut p_head: Option<Box<ListNode>> = None;
        let mut prev_tail = &mut p_head;
        let mut k_group: Option<Box<ListNode>> = None;
        loop {
            for k_group_len in 0..k {
                if let Some(mut node) = head {
                    head = node.next.take();
                    node.next = k_group;
                    k_group = Some(node);
                } else {
                    let mut reverted_k_group: Option<Box<ListNode>> = None;
                    for _ in 0..k_group_len {
                        let mut node = k_group.unwrap();
                        k_group = node.next.take();
                        node.next = reverted_k_group;
                        reverted_k_group = Some(node);
                    }
                    *prev_tail = reverted_k_group;
                    return p_head;
                }
            }
            *prev_tail = k_group;
            for _ in 0..k {
                prev_tail = &mut prev_tail.as_mut().unwrap().next;
            }
            k_group = None;
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

struct Solution;
struct Solution2;

fn main() {}
