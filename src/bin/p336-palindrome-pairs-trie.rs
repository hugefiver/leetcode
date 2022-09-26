use std::collections::HashMap;
use std::hash::Hash;

struct Node<'w, T> {
    next: HashMap<T, Self>,
    value: Option<(usize, &'w [T])>,
}

impl<'w, T> Default for Node<'w, T> {
    fn default() -> Self {
        Node::new()
    }
}

impl<'w, T> Node<'w, T> {
    pub fn new() -> Self {
        Self {
            next: HashMap::new(),
            value: None,
        }
    }
}

impl<'w, T: Copy + Eq + Hash> Node<'w, T> {
    pub fn insert_rev(&mut self, index: usize, word: &'w [T]) {
        let mut node = self;

        for &x in word.iter().rev() {
            node = node.next.entry(x).or_default();
        }

        node.value = Some((index, word));
    }

    fn pairs(&self, answer: &mut Vec<Vec<i32>>, index: usize, word: &[T]) {
        // Corner case: The root node contains the "empty string"
        if let Some((idx, _value)) = self.value.clone() {
            if index != idx && is_palindrome(word, 0, word.len() - 1) {
                answer.push(vec![index as i32, idx as i32]);
            }
        }

        let mut node = self;
        for pos in 0..word.len() {
            node = match node.next.get(&word[pos]) {
                None => return,
                Some(next) => next,
            };

            // If the new node is a word, then we must check if the
            // remainder of the current word is palindrome
            if let Some((idx, _value)) = node.value.clone() {
                let to = word.len() - 1;
                if index != idx && (pos == to || is_palindrome(word, pos + 1, to)) {
                    answer.push(vec![index as i32, idx as i32]);
                }
            }
        }

        // We've processed all letters from the current word. Now we have
        // to check if we can concatenate it to a longer word and still produce
        // a palindrome
        dfs(node, answer, index, word.len() + 1);
    }
}

fn dfs<'w, T: Copy + Eq + Hash>(
    node: &'w Node<T>,
    answer: &mut Vec<Vec<i32>>,
    index: usize,
    skip_len: usize,
) {
    for (_, next) in node.next.iter() {
        if let Some((idx, value)) = next.value.clone() {
            if index != idx && is_palindrome(value, 0, value.len() - skip_len) {
                answer.push(vec![index as i32, idx as i32]);
            }
        }

        dfs(next, answer, index, skip_len);
    }
}

fn is_palindrome<T: Eq>(word: &[T], from: usize, to: usize) -> bool {
    assert!(from <= to);
    let mut lo = from;
    let mut hi = to;

    while lo < hi {
        if word[lo] != word[hi] {
            return false;
        }

        lo += 1;
        hi -= 1;
    }

    true
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie = Node::new();
        for (index, word) in words.iter().enumerate() {
            trie.insert_rev(index, word.as_bytes());
        }

        let mut answer = vec![];
        for (index, word) in words.iter().enumerate() {
            trie.pairs(&mut answer, index, word.as_bytes());
        }

        answer
    }
}

struct Solution;

fn main() {}
