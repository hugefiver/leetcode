use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    indx: Vec<i32>,
    word: i32
}

impl Trie {
    fn new() -> Self {
        Self { children: HashMap::new(), indx: vec![], word: -1 }
    }
    
    fn build(&mut self, words: &Vec<Vec<char>>, empty: &mut i32) {
        let n = words.len();
        
        for i in 0..n {
            if words[i].len() == 0 {
                *empty = i as i32;
                continue
            }
            
            let mut node = &mut *self;
            for c in &words[i] {
                node = node.children.entry(*c).or_insert(Trie::new());
                node.indx.push(i as i32);
            }
            
            node.word = i as i32;
        }
    }
    
    fn palindromic(data: &Vec<char>, start: usize, end: usize) -> bool {
        for i in 0..end - start {
            if start + i >= end - 1 - i { break }
            if data[start + i] != data[end - 1 - i] { return false }
        }
        true
    } 
    
    fn calculate(&self, words: &Vec<Vec<char>>, empty: i32) -> Vec<Vec<i32>> {
        let n = words.len();
        let mut ret: Vec<Vec<i32>> = vec![];
        
        if empty != -1 {
            for i in 0..n {
                if i as i32 == empty { continue }
                if Self::palindromic(&words[i], 0, words[i].len()) == false { continue }
                ret.push(vec![empty, i as i32]);
                ret.push(vec![i as i32, empty]);
            } 
        }
        
        for i in 0..n {
            let mut node = self;
            let m = words[i].len();
            
            for j in (0..m).rev() {
                let c = words[i][j];
                if node.children.contains_key(&c) == false { break }
                
                node = node.children.get(&c).unwrap();
                if node.word != -1 && node.word != i as i32 {
                    if Self::palindromic(&words[i], 0, j) {
                        ret.push(vec![node.word, i as i32]);
                    }
                }
                if j == 0 {
                    for k in &node.indx {
                        let k = *k as usize;
                        if k == i || k as i32 == node.word { continue }
                        if Self::palindromic(&words[k], m, words[k].len()) == false { continue }
                        ret.push(vec![k as i32, i as i32]);
                    }
                }
            }
        }
        ret
    }
}


impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie = Trie::new();
        let mut empty = -1;
        let words: Vec<Vec<char>> = words.iter().map(|w| w.chars().collect::<Vec<char>>()).collect();
        
        trie.build(&words, &mut empty);
        trie.calculate(&words, empty)
    }
}

impl Solution {
    pub fn palindrome_pairs_4(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut rev_fulls : HashMap<&[u8], i32> = HashMap::new();
        let mut rev_prefixes : HashMap<&[u8], Vec<i32>> = HashMap::new();
        let mut rev_suffixes : HashMap<&[u8], Vec<i32>> = HashMap::new();
        let wl = words.len();
        
        let mut min_len = 1000; // words[i].length < 300, wo should be enough
        let mut max_len = 0;
        for i in 0..wl {
            min_len = usize::min(min_len, words[i].len());
            max_len = usize::max(max_len, words[i].len());
        }
        // println!("{}-{}", min_len, max_len);
        
        for i in 0..wl {
            let word = &(words[i as usize]);
            let bs = word.as_bytes();
            rev_fulls.insert(&bs[..], i as i32);
            
            let l = bs.len();
            let min_keep = min_len;
            let max_keep = usize::min(l - 1, max_len);
            // println!("{}, {}-{}", word, min_keep, max_keep);
            for d in (l - max_keep)..(l - min_keep + 1) {
                let mut ok = true;
                for j in 0..(d / 2) {
                    if bs[l - 1 - j] != bs[l - d + j] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                rev_prefixes.entry(&bs[..(l-d)]).and_modify(|x| x.push(i as i32)).or_insert(vec![i as i32]);
            }
            
            for d in (l - max_keep)..(l - min_keep + 1) {
                let mut ok = true;
                for j in 0..(d / 2) {
                    if bs[j] != bs[d - 1 - j] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                rev_suffixes.entry(&bs[d..]).and_modify(|x| x.push(i as i32)).or_insert(vec![i as i32]);
            }
        }
        
        // println!("{:?}", rev_fulls);
        // println!("{:?}", rev_prefixes);
        // println!("{:?}", rev_suffixes);
        
        let mut res : Vec<Vec<i32>> = Vec::new();
        
        for i in 0..(wl as i32) {
            let word = &(words[i as usize]);
            let mut bytes = word.clone().into_bytes();
            bytes.reverse();
            if let Some(j) = rev_fulls.get(&bytes[..]) {
                if *j != i {
                    res.push(vec![*j, i]);
                }
            }
            if let Some(t) = rev_prefixes.get(&bytes[..]) {
                for j in t {
                    if *j != i {
                        res.push(vec![*j, i]);
                    }
                }
            }
            if let Some(t) = rev_suffixes.get(&bytes[..]) {
                for j in t {
                    if *j != i as i32 {
                        res.push(vec![i, *j]);
                    }
                }
            }
        }
        
        res
    }
}

impl Solution {
    pub fn palindrome_pairs_3(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let word_map: HashMap<_, _> = words
            .iter()
            .enumerate()
            .map(|(i, w)| (w.as_str(), i))
            .collect();

        words.iter().enumerate().fold(vec![], |mut acc, (i, w)| {
            let n = w.len();
            let rev: String = w.chars().rev().collect();
            let from_right = (0..n)
                .filter(|&k| w[..n - k].chars().eq(w[..n - k].chars().rev()))
                .filter_map(|k| word_map.get(&rev[..k]).map(|&j| (j, i)));

            (0..=n)
                .filter(|&k| w[k..].chars().eq(w[k..].chars().rev()))
                .filter_map(|k| word_map.get(&rev[n - k..]).map(|&j| (i, j)))
                .chain(from_right)
                .filter(|&(i, j)| i != j)
                .for_each(|(i, j)| acc.push(vec![i as _, j as _]));

            acc
        })
    }

    pub fn palindrome_pairs_2(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut ms: Vec<(HashSet<String>, HashSet<String>)> =
            vec![(HashSet::new(), HashSet::new()); words.len()];
        let mut ret = vec![];

        for (idx, s) in words.iter().enumerate() {
            let (fset, bset) = &mut ms[idx];
            let len = s.len();
            for i in 0..len {
                fset.insert(s[..i].chars().rev().collect());
                bset.insert(s[i..].chars().rev().collect());
            }
        }

        for (i, s1) in words.iter().enumerate() {
            for (j, s2) in words.iter().enumerate() {
                if i == j {
                    continue;
                }
                let len = s1.len() + s2.len();
                let mut mid = len / 2;
                let odd = len % 2 == 1;
                if {
                    if len <= 1 {
                        true
                    } else if mid == s1.len() {
                        ms[j].1.contains(s1)
                    } else if mid < s1.len() {
                        if odd {
                            mid += 1;
                        }
                        ms[i].0.contains(&(s1[mid..].to_owned() + s2))
                    } else {
                        mid -= s1.len();
                        ms[j].1.contains(&(s1.to_owned() + &s2[..mid]))
                    }
                } {
                    ret.push(vec![i as i32, j as i32]);
                }
            }
        }

        ret
    }

    pub fn palindrome_pairs_1(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if Self::is_palindrome(&words[i], &words[j]) {
                    ret.push(vec![i as i32, j as i32]);
                }
                if Self::is_palindrome(&words[j], &words[i]) {
                    ret.push(vec![j as i32, i as i32]);
                }
            }
        }
        ret
    }

    fn is_palindrome(s1: &str, s2: &str) -> bool {
        s1.chars()
            .chain(s2.chars())
            .zip(s2.chars().rev().chain(s1.chars().rev()))
            .take((s1.len() + s2.len()) / 2)
            .all(|(x, y)| x == y)
    }
}

#[test]
fn test() {}

struct Solution;

fn main() {}

/* Java Trie

Thought

We want to concatenate string B to string A to make AB palindrome.

How could AB be palindrome? 
If B ends with x, then A must starts with x. If the second character of B is y, then the second last character of A is y...
That is,
  Case 1. A must be prefix of reversed B, and the rest of reversed B should be palindrome. For example,
	(B:oooabc - cbaooo,    A:cba       AB:cba|oooabc)
  Case 2. Or, reversed B must be prefix of A, and the rest of A should be palindrome. For example,
	(B:abc - cba           A:cbaooo,   AB:cbaooo|abc)
    
Each word in words can be B. We put all reversed words in a trie. 
Each word in words can be A. So we search A in trie, 
In this way,
  Case 1. if we found A in trie, and the branch under the end node is a palindrome, we found it!
  Case 2. if we reach a leaf of trie, and the rest of A is palindrome, we found it! 
  
  For Case 1., we modify TrieNode data structure by adding belowPalindromeWordIds - list of word indices such that nodes below can construct a palindrome.
  For Case 2., we create a method isPalindrome(str, start, end) .

Please take care of corner cases of empty string. Both ("", self-palindrome) and (self-palindrome, "") are still palindrome.
Code

    private Node root;
    
    public List<List<Integer>> palindromePairs(String[] words) {
        
        if (words == null || words.length == 0)
            return new ArrayList<>();
        
        root = new Node();
        int n = words.length;
        List<List<Integer>> finalResult = new ArrayList<>();
        
        for (int i = 0; i < n; i++) {
            if (words[i].isEmpty()) {
                // Pair with all self-palindrome.
                List<Integer> selfPalindromeWordIndices = getSelfPalindrome(words);
                for (int pairId : selfPalindromeWordIndices) {
                    finalResult.add(new ArrayList<>(Arrays.asList(i, pairId)));
                    finalResult.add(new ArrayList<>(Arrays.asList(pairId, i)));
                }
            }
            else {
                insert(reverse(words[i]), i);
            }
        }
        for (int i = 0; i < n; i++) {
            List<Integer> wordIndices = search(words[i], i);
            for (int pairId : wordIndices) {
                finalResult.add(new ArrayList<>(Arrays.asList(i, pairId)));
            }
        }
        
        return finalResult;
    }
    
    /****************** Trie-related *******************/
    
    private List<Integer> search(String word, int index) {
        
        List<Integer> wordIndices = new ArrayList<>();
        Node ptr = root;
        int n = word.length();
        for (int i = 0; i < n; i++) {
            int label = word.charAt(i) - 'a';
            if (ptr.endWordId > -1 && isPalindrome(word, i, n - 1)) {
                wordIndices.add(ptr.endWordId);                
            }
            if (ptr.children[label] == null) {
                return wordIndices;
            } 
            ptr = ptr.children[label];
        }
        if (ptr.endWordId > -1 && ptr.endWordId != index)
            wordIndices.add(ptr.endWordId);
        if (!ptr.belowPalindromeWordIds.isEmpty())
            wordIndices.addAll(ptr.belowPalindromeWordIds);
        
        return wordIndices;
    }
    
    private void insert(String word, int index) {
        
        Node ptr = root;
        int n = word.length();
        for (int i = 0; i < n; i++) {
            int label = word.charAt(i) - 'a';               
            if (ptr.children[label] == null)
                ptr.children[label] = new Node();
            ptr = ptr.children[label];
            if (isPalindrome(word, i + 1, n - 1))
                ptr.belowPalindromeWordIds.add(index);
        }
        
        ptr.endWordId = index;
    }
    
    class Node {
        
        Node[] children;
        int endWordId; // Equals to -1 in default. If it is a word's end, it is the index of the word.
        List<Integer> belowPalindromeWordIds; // List of word indices such that nodes below can construct a palindrome.
        
        public Node() {
            children = new Node[26];
            endWordId = -1;
            belowPalindromeWordIds = new ArrayList<>();
        }
        
    }
    
    /****************** Utility *******************/
    
    private String reverse(String str) {
        return new StringBuilder(str).reverse().toString();
    }
    
    private boolean isPalindrome(String str, int start, int end) {
        
        if (start > end) {
            return false;
        }
        
        while (start < end) {
            if (str.charAt(start) != str.charAt(end))
                return false;
            start++;
            end--;
        }
        
        return true;
    }
    
    private List<Integer> getSelfPalindrome(String[] words) {
        List<Integer> wordIndices = new ArrayList<>();
        for (int i = 0; i < words.length; i++) {
            if (isPalindrome(words[i], 0, words[i].length() - 1)) {
                wordIndices.add(i);
            }
        }
        return wordIndices;
    }

*/

/* C++ Trie

struct TrieNode {
    TrieNode *next[26] = {};
    int index = -1;
    vector<int> palindromeIndexes;
};

class Solution {
    TrieNode root; // Suffix trie
    void add(string &s, int i) {
        auto node = &root;
        for (int j = s.size() - 1; j >= 0; --j) {
            if (isPalindrome(s, 0, j)) node->palindromeIndexes.push_back(i); // A[i]'s prefix forms a palindrome
            int c = s[j] - 'a';
            if (!node->next[c]) node->next[c] = new TrieNode();
            node = node->next[c];
        }
        node->index = i;
        node->palindromeIndexes.push_back(i); // A[i]'s prefix is empty string here, which is a palindrome.
    }
    
    bool isPalindrome(string &s, int i, int j) {
        while (i < j && s[i] == s[j]) ++i, --j;
        return i >= j;
    }
    
public:
    vector<vector<int>> palindromePairs(vector<string>& A) {
        int N = A.size();
        for (int i = 0; i < N; ++i) add(A[i], i);
        vector<vector<int>> ans;
        for (int i = 0; i < N; ++i) {
            auto s = A[i];
            auto node = &root;
            for (int j = 0; j < s.size() && node; ++j) {
                if (node->index != -1 && node->index != i && isPalindrome(s, j, s.size() - 1)) ans.push_back({ i, node->index }); 
                // A[i]'s prefix matches this word and A[i]'s suffix forms a palindrome
                node = node->next[s[j] - 'a'];
            }
            if (!node) continue;
            for (int j : node->palindromeIndexes) { 
                // A[i] is exhausted in the matching above. 
                // If a word whose prefix is palindrome after matching its suffix with A[i], 
                // then this is also a valid pair
                if (i != j) ans.push_back({ i, j });
            }
        }
        return ans;
    }
};

*/

/* Rust

fn mk_idx(v: u8)->usize{
    (v-b'a') as usize
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
	    // Need this ugly initialization because [x; N] require x to be copyable :(
		// Mapping from last letter to indices in `words`
        let mut last_let_to_indices = [
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new(),Vec::new(),
            Vec::new(),Vec::new()
        ];
		// Flag for possible empty string
        let mut empty_idx = None;
        for (i, w) in words.iter().map(String::as_bytes).enumerate(){
            match w.last(){
                Some(&c)=>last_let_to_indices[mk_idx(c)].push(i),
                None=>empty_idx = Some(i),
            }
        }
        
        let mut results = Vec::new();
        // Improved O(n^2)
        for (i, w1) in words.iter().map(String::as_bytes).enumerate(){
		    // We will handle it later
            if w1.is_empty(){
                continue;
            }
            let first = mk_idx(*w1.first().unwrap());
            for &j in last_let_to_indices[first].iter(){
                if i==j{
                    continue;
                }
                let w2 = words[j].as_bytes();
                let sum_len_half =(w1.len() + w2.len())/2;
				// Rust iterators power allows to avoid extra allocation here
                let iter = w1.iter().copied().chain(w2.iter().copied());
                let reversed = iter.clone().rev();
                for (idx, (c1, c2)) in iter.zip(reversed).enumerate(){
                    if c1 != c2{
                        break;
                    }
                    if idx >= sum_len_half{
                        results.push(vec![i as i32, j as i32]);
                        break;
                    }
                }
            }
        }
        
        // Case when one of words is empty
        // O(n)
        if let Some(empty_idx) = empty_idx {
            for (i, w) in words.iter().map(String::as_bytes).enumerate(){
                if i==empty_idx{
                    continue;
                }
                let len_half = w.len() / 2;
                let iter = w.iter().copied();
                let reversed = iter.clone().rev();
                for (idx, (c1, c2)) in iter.zip(reversed).enumerate(){
                    if c1 != c2{
                        break;
                    }
                    if idx >= len_half{
                        results.push(vec![i as i32, empty_idx as i32]);
                        results.push(vec![empty_idx as i32, i as i32]);
                        break;
                    }
                }
            }
        }
        
        results
    }
}

*/