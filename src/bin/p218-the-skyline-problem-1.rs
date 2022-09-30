#[derive(Debug)]
struct SkyLine {
    height: i32,
    left: i32,
    right: i32,
}

impl Ord for SkyLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&(other.height))
    }
}

impl PartialOrd for SkyLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SkyLine {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl Eq for SkyLine {}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<SkyLine> = BinaryHeap::new();
        let mut out: Vec<Vec<i32>> = Vec::new();
        let mut y = 0;
        let mut y2 = 0;
        for sky in buildings.iter().map(|b| SkyLine {
            left: b[0],
            right: b[1],
            height: b[2],
        }) {
            let left = sky.left;

            // pop all outdated ones
            loop {
                match heap.peek() {
                    Some(current) => {
                        let r = current.right;
                        if r < left {
                            heap.pop();
                            y2 = 0;
                            loop {
                                if let Some(sub_sky) = heap.peek() {
                                    if sub_sky.right < r {
                                        heap.pop();
                                    } else {
                                        y2 = sub_sky.height;
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            if y2 < y {
                                y = y2;
                                if let Some(last) = out.last_mut() {
                                    if last[0] == r {
                                        last[1] = y;
                                        continue;
                                    }
                                }
                                out.push(vec![r, y]);
                            }
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }
            heap.push(sky);
            let peak = heap.peek().unwrap();
            y2 = peak.height;
            if y2 > y {
                y = y2;
                if let Some(last) = out.last_mut() {
                    if last[0] == left {
                        last[1] = y2;
                        continue;
                    }
                }
                out.push(vec![left, y]);
            }
        }

        // pop the remainder
        loop {
            match heap.pop() {
                Some(current) => {
                    let r = current.right;
                    y2 = 0;
                    loop {
                        if let Some(sub_sky) = heap.peek() {
                            if sub_sky.right < r {
                                heap.pop();
                            } else {
                                y2 = sub_sky.height;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    if y2 < y {
                        y = y2;
                        if let Some(last) = out.last_mut() {
                            if last[0] == r {
                                last[1] = y;
                                continue;
                            }
                        }

                        out.push(vec![r, y]);
                    }
                }
                None => break,
            }
        }

        out
    }

}

struct Solution;

fn main() {}
