use std::collections::HashMap;
use std::collections::VecDeque;

struct Node {
    index: Option<(usize, usize)>,
    children: HashMap<char, Node>,
    suffix: Option<*const Node>,
}

impl Node {
    fn new() -> Self {
        Node{
            index: None,
            children: HashMap::new(),
            suffix: None,
        }
    }
    
    fn add_word<I>(&mut self, index: usize, mut word: I)
            where I: Iterator<Item = char> {
        match word.next() {
            Some(c) => {
                let child = self.children.entry(c).or_insert_with(||
                    Self::new()
                );
                child.add_word(index, word);
            },
            None => {
                if let Some((index, count)) = self.index {
                    self.index = Some((index, count + 1));
                } else {
                    self.index = Some((index, 1))
                }
            }
        }
    }
    
    fn build_suffixes(&mut self) {
        let mut node_queue: VecDeque<&mut Self> = VecDeque::new();
        let root = self as *const Node;
        for child in self.children.values_mut() {
            child.suffix = Some(root);
            node_queue.push_back(child);
        }
        while let Some(parent) = node_queue.pop_front() {
            for (c, child) in parent.children.iter_mut() {
                child.suffix = Some(root);
                let mut suffix_node: Option<*const Node> = parent.suffix.clone();
                while let Some(suffix) = suffix_node {
                    unsafe {
                        if let Some(suffix_child) = (*suffix).children.get(c) {
                            child.suffix = Some(suffix_child);
                            break;
                        }
                        suffix_node = (*suffix).suffix;
                    }
                }
                node_queue.push_back(child);
            }
        }
    }

    fn transition<'a>(&'a self, from: &'a Self, c: char) -> &'a Self {
        if let Some(child) = from.children.get(&c) {
            child
        } else if let Some(suffix) = from.suffix {
            self.transition(unsafe { &*suffix }, c)
        } else {
            self
        }
    }
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let m = words.len();
        if m == 0 {
            return Vec::new();
        }
        let k = words[0].len();
        
        let mut output: Vec<i32> = Vec::new();
        let mut matched_queue: Vec<VecDeque<usize>> =
            vec![VecDeque::with_capacity(m); k];
        let mut matched: Vec<Vec<usize>> =
            vec![vec![0; m]; k];
        
        let mut dict: Node = Node::new();
        let mut words = words;
        for (i, word) in words.iter_mut().enumerate() {
            dict.add_word(i, word.drain(..));
        }
        dict.build_suffixes();
        
        let total_length = m * k;
        let mut pos: &Node = &dict;
        for (i, c) in s.chars().enumerate() {
            pos = dict.transition(pos, c);
            let offset = i % k;
            let matched_queue = &mut matched_queue[offset];
            let matched = &mut matched[offset];
            if let Some((index, count)) = pos.index {
                if count <= matched[index] {
                    while let Some(front_index) = matched_queue.pop_front() {
                        matched[front_index] -= 1;
                        if front_index == index {
                            break;
                        }
                    }
                }
                matched[index] += 1;
                matched_queue.push_back(index);
                if matched_queue.len() == m {
                    output.push((i + 1 - total_length) as i32);
                }
            } else {
                while let Some(index) = matched_queue.pop_front() {
                    matched[index] -= 1;
                }
            }
        }
        
        output
    }
}

struct Solution;

fn main() {}