// Something goes wrong
fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut segt = SegmentTree::new();
    intervals.into_iter().for_each(|&r| segt.insert(r));
    segt.travel_leaf()
}

struct SegmentTree {
    root: Option<Box<Node>>,
}

struct Node {
    range: (i32, i32),
    // sum: i32,
    mid: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(range: (i32, i32)) -> Self {
        Node {
            range,
            // sum: 0,
            mid: range.0,
            left: None,
            right: None,
        }
    }
}

impl SegmentTree {
    fn new() -> Self {
        SegmentTree { root: None }
    }

    fn insert(&mut self, (i, j): (i32, i32)) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new((i, j))));
            return;
        }
        Self::insert_through(&mut self.root, (i, j));
    }

    fn insert_through(root: &mut Option<Box<Node>>, (l, r): (i32, i32)) {
        if let Some(node) = root {
            let (nl, nr) = node.range;
            if node.left.is_none() && node.right.is_none() {
                if l >= nl && r <= nr {
                    return;
                } else if l <= nl && r >= nr {
                    node.range = (l, r);
                    node.mid = l;
                } else if r <= nl {
                    node.mid = r;
                    node.left = Box::new(Node::new((l, r))).into();
                    node.right = Box::new(Node::new(node.range)).into();
                    node.range.0 = l;
                } else if l >= nr {
                    node.mid = l;
                    node.right = Box::new(Node::new((l, r))).into();
                    node.left = Box::new(Node::new(node.range)).into();
                    node.range.1 = r;
                } else {
                    let (l, r) = (l.min(nl), r.max(nr));
                    let mid = (l + r) / 2;
                    node.mid = mid;
                    node.left = Box::new(Node::new((l, mid))).into();
                    node.right = Box::new(Node::new((mid, r))).into();
                }
            } else {
                if l < node.mid {
                    Self::insert_through(&mut node.left, (l, node.mid));
                }

                if r > node.mid {
                    Self::insert_through(&mut node.right, (node.mid, r));
                }
            }
        } else {
            root.replace(Box::new(Node::new((l, r))));
        }
    }

    fn travel_leaf(&self) -> i32 {
        fn foo(node: &Box<Node>) -> i32 {
            if node.left.is_none() && node.right.is_none() {
                node.range.1 - node.range.0
            } else {
                foo(node.left.as_ref().unwrap()) + foo(node.right.as_ref().unwrap())
            }
        }
        if let Some(ref root) = self.root {
            foo(&root)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod sample_tests {
    use super::*;
    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
    }

    #[test]
    fn large_intervals() {
        assert_eq!(
            sum_intervals(&[(-1_000_000_000, 1_000_000_000)]),
            2_000_000_000,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]),
            100_000_030,
            "{}",
            ERR_MSG
        );
    }
}
