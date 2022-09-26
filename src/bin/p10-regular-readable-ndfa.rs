struct Solution {}
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // build the pattern matcher
        let mut automaton: NDFA = NDFA::new(p);

        automaton.is_match(s)
    }
}

#[derive(Clone)]
pub enum Pattern {
    AnyStarred,
    AnyCount(i64),
    CharStarred(char),
    CharCount(char, i64),
}

#[derive(Hash, Eq, PartialEq)]
enum Action {
    Any,
    Skip,
    Char(char),
}

struct State {
    /// maps an input to the next state
    actions: HashMap<Action, usize>,
}
impl State {
    pub fn new(actions: HashMap<Action, usize>) -> Self {
        State { actions }
    }
    /// Returns a vector of the possible next states based on an input.
    /// The input action here is usually expected to be a Action::Char
    pub fn get(&self, action: &Action) -> Vec<usize> {
        let next = self.actions.get(action);
        let mut ret = Vec::new();
        if let Some(&next) = next {
            ret.push(next);
        }
        let any = self.actions.get(&Action::Any);
        if let Some(&any) = any {
            ret.push(any);
        }
        ret
    }
    /// If the state can be skipped, returns the next state
    pub fn skip(&self) -> Option<&usize> {
        self.actions.get(&Action::Skip)
    }
}

pub struct NDFA {
    /// The states of the automata
    states: Vec<State>,
    /// The current possible states of the automata based on the input string
    current: HashSet<usize>,
}

impl NDFA {
    pub fn new(pattern: String) -> Self {
        let pattern = Self::parse_pattern(pattern);

        let states = Self::build_states(pattern);

        let mut automata = NDFA {
            states,
            current: HashSet::default(),
        };
        automata.init_state();
        automata
    }
    /// parses the pattern string into a usable input
    fn parse_pattern(pattern: String) -> Vec<Pattern> {
        let mut p_pattern: Vec<Pattern> = Vec::new();
        for c in pattern.chars() {
            match c {
                '.' => p_pattern.push(Pattern::AnyCount(1)),
                '*' => {
                    let last = p_pattern.len() - 1;
                    let m = p_pattern[last].clone();
                    match m {
                        Pattern::AnyCount(cn) if cn == 1 => p_pattern[last] = Pattern::AnyStarred,
                        Pattern::AnyCount(cn) if cn > 1 => {
                            p_pattern[last] = Pattern::AnyCount(cn - 1);
                            p_pattern.push(Pattern::AnyStarred)
                        }
                        Pattern::CharCount(ch, cn) if cn == 1 => {
                            p_pattern[last] = Pattern::CharStarred(ch)
                        }
                        Pattern::CharCount(ch, cn) if cn > 1 => {
                            p_pattern[last] = Pattern::CharCount(ch, cn - 1);
                            p_pattern.push(Pattern::CharStarred(ch))
                        }
                        _ => {}
                    }
                }
                _ => {
                    let m = p_pattern.last();
                    match m {
                        Some(Pattern::CharCount(ch, count)) if *ch == c => {
                            let l = p_pattern.len() - 1;
                            p_pattern[l] = Pattern::CharCount(c, count + 1);
                        }
                        _ => p_pattern.push(Pattern::CharCount(c, 1)),
                    }
                }
            }
        }
        p_pattern
    }
    /// builds the states of the automata
    fn build_states(pattern: Vec<Pattern>) -> Vec<State> {
        let mut states = vec![];

        for pat in pattern {
            match pat {
                Pattern::AnyCount(count) => {
                    for _ in 0..count {
                        let next_state = states.len() + 1;
                        states.push(State::new({
                            let mut hm = HashMap::new();
                            hm.insert(Action::Any, next_state);
                            hm
                        }))
                    }
                }
                Pattern::AnyStarred => states.push(State::new({
                    let cur_state = states.len();
                    let mut hm = HashMap::new();
                    hm.insert(Action::Any, cur_state);
                    hm.insert(Action::Skip, cur_state + 1);
                    hm
                })),
                Pattern::CharCount(ch, count) => {
                    for _ in 0..count {
                        let next_state = states.len() + 1;
                        states.push(State::new({
                            let mut hm = HashMap::new();
                            hm.insert(Action::Char(ch), next_state);
                            hm
                        }))
                    }
                }
                Pattern::CharStarred(ch) => states.push(State::new({
                    let cur_state = states.len();
                    let mut hm = HashMap::new();
                    hm.insert(Action::Char(ch), cur_state);
                    hm.insert(Action::Skip, cur_state + 1);
                    hm
                })),
            }
        }
        //final state
        states.push(State::new(HashMap::new()));
        states
    }
    /// initializes the automata by adding all the possible initial states
    fn init_state(&mut self) {
        let mut hs = HashSet::new();
        self.grab_states(0, &mut hs);
        self.current = hs;
    }
    /// returns whether the input string matches the regex
    pub fn is_match(&mut self, string: String) -> bool {
        self.run(string);
        let res = self.reached_final();
        // reset the automata to the initial states
        self.init_state();
        res
    }
    /// processes the input string through the automata
    fn run(&mut self, string: String) {
        for c in string.chars() {
            self.step(c);
            if self.current.is_empty() {
                break;
            }
        }
    }
    /// changes the current states to the next possible current states based on the input character
    fn step(&mut self, c: char) {
        let mut states = HashSet::new();
        for state in self.current.iter().copied() {
            if state < self.states.len() {
                for s in self.states[state].get(&Action::Char(c)) {
                    self.grab_states(s, &mut states)
                }
            }
        }
        self.current = states;
    }
    /// adds the state and all the other states that can be reached from it without an input (aka if it's skippable)
    fn grab_states(&self, state_idx: usize, states: &mut HashSet<usize>) {
        if state_idx < self.states.len() && !states.contains(&state_idx) {
            states.insert(state_idx);
            if let Some(&next) = self.states[state_idx].skip() {
                self.grab_states(next, states)
            }
        }
    }
    /// returns whether the automata has reached the final state
    fn reached_final(&self) -> bool {
        self.current.contains(&(self.states.len() - 1))
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
