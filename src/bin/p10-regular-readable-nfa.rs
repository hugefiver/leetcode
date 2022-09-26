struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Traverse NFA while going monotonically left-to-right through `s` by
        // tracking all states that one can be in after reaching a character in `s`.
        // - States are represented by indices in 0..=p.len().
        // - The same index also corresponds to the transition out of that state.

        // Track intermediate states using sets instead of arrays to de-dupe states
        // to avoid an exponential explosion when matching, e.g., "aaaaaaaaaaaa"
        // against "a*a*a*".
        let mut curr_states = HashSet::with_capacity(p.len() + 1);
        let mut next_states = HashSet::with_capacity(p.len() + 1);

        let s = s.into_bytes();
        let pat = Pattern::new(p);

        // Start with all initial states that match an empty string.
        pat.insert_all_reachable(&mut curr_states, /* state= */ 0usize);

        // Pass characters one by one through the pattern state machine.
        for ch in s {
            next_states.clear();
            for &state in &curr_states {
                if pat.matches(state, ch) {
                    // Repeat-pattern means that we can stay in the same state.
                    if pat.is_repeat(state) {
                        next_states.insert(state);
                    }
                    // We can also proceed forward at least one step.
                    pat.insert_all_reachable(&mut next_states, pat.next_state(state));
                }
            }
            std::mem::swap(&mut curr_states, &mut next_states);
            if curr_states.is_empty() {
                return false;
            }
        }

        // See if any traversals ended exactly at the final state.
        curr_states.iter().any(|&state| pat.at_final_state(state))
    }
}

use std::collections::HashSet;
struct Pattern(Vec<u8>);
impl Pattern {
    fn new(p: String) -> Self {
        Pattern(p.into_bytes())
    }
    
    fn matches(&self, state: usize, ch: u8) -> bool {
        state < self.0.len() && (self.0[state] == b'.' || self.0[state] == ch)
    }
    
    fn is_repeat(&self, state: usize) -> bool {
        state + 1 < self.0.len() && self.0[state + 1] == b'*'
    }

    fn next_state(&self, state: usize) -> usize {
        if self.is_repeat(state) { state + 2 } else { state + 1 }
    }
    
    /// Insert `state` and all states reachable while matching an empty string.
    fn insert_all_reachable(&self, states: &mut HashSet<usize>, mut state: usize) {
        states.insert(state);
        while self.is_repeat(state) { 
            state = self.next_state(state); 
            states.insert(state);
        }
    }
    
    fn at_final_state(&self, state: usize) -> bool {
        state == self.0.len()
    }
}

#[test]
fn test() {
    let suits = vec![("aa", "a", false), ("aa", "a*", true)];

    for (s, p, matches) in suits {
        assert_eq!(Solution::is_match(s.to_string(), p.to_string()), matches);
    }
}

fn main() {}
