use std::mem::swap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        MatcherNfa::new(p).full_match(s)
    }
}

/// Represents Nondeterministic Finite Automata (state machines) constructed
/// using a regular expression, and used to determine if strings match it.
///
struct MatcherNfa {
    start: HState,
    states: States,
    set_id: usize,
}

impl MatcherNfa {
    /// Creates a NFA state machine from a regular expression. Internally, the
    /// expression is converted to its postfix representation before building
    /// the NFA. Internally, '\0' is treated as the postfix concatenation
    /// operator.
    ///
    fn new(p: String) -> Self {
        use Transition::*;
        let mut states = States::new();
        let mut stack = Vec::<Frag>::new();
        let postfix = re2postfix(p);

        for ch in postfix {
            match ch {
                '\0' => {
                    // Handle postfix concatenation operator.
                    let e2 = stack.pop().unwrap();
                    let e1 = stack.pop().unwrap();
                    e1.out.patch(e2.start, &mut states);
                    stack.push(Frag::new(e1.start, e2.out));
                }
                '*' => {
                    // Handle '*' regular expression operator.
                    let e = stack.pop().unwrap();
                    let s = states.new_state(Split, Some(e.start), None);
                    e.out.patch(s, &mut states);
                    stack.push(Frag::new(s, s.out_n_handle(2).into()));
                }
                _ => {
                    // Treat all other characters as transition match chars.
                    let s = states.new_state(Char(ch), None, None);
                    stack.push(Frag::new(s, s.out_n_handle(1).into()));
                }
            }
        }
        let e = stack.pop().unwrap();
        let m = states.new_state(Match, None, None);

        e.out.patch(m, &mut states);

        MatcherNfa {
            start: e.start,
            states,
            set_id: 0,
        }
    }
    /// Run NFA to determine whether it matches `s`. Not a partial match.
    ///
    fn full_match(&mut self, s: String) -> bool {
        let mut curr_set = FollowSet::new();
        let mut next_set = FollowSet::new();

        curr_set.start(self.start, &mut self.states, self.set_id);

        for ch in s.chars() {
            curr_set.step(ch, &mut next_set, &mut self.states, &mut self.set_id);

            swap(&mut curr_set, &mut next_set);

            if curr_set.count() == 0 {
                break;
            }
        }
        curr_set.is_match(&self.states)
    }
}

/// NFA state node.
///
#[derive(Debug)]
struct State {
    c: Transition,
    out_1: Option<HState>,
    out_2: Option<HState>,
    last_set: usize,
}

/// Handle for State objects.
///
#[derive(Copy, Clone, Debug)]
struct HState {
    idx: usize,
}
impl HState {
    /// Create a new handle to a specific field of a State object.
    ///
    fn out_n_handle(&self, hout_n: u8) -> HStateField {
        HStateField {
            state: *self,
            which: hout_n,
        }
    }
}

/// A handle to the field of a State object. The associated fields are
/// `state.out_1` or `state.out_2`.
///
#[derive(Copy, Clone, Debug)]
struct HStateField {
    state: HState,
    which: u8,
}

/// Contiguous memory storage for State objects.
///
struct States {
    mem: Vec<State>,
}
impl States {
    /// Create a new States object to hold State objects.
    ///
    fn new() -> Self {
        States { mem: vec![] }
    }
    /// Creates a new State object and returns its handle.
    ///
    fn new_state(&mut self, c: Transition, out_1: Option<HState>, out_2: Option<HState>) -> HState {
        self.mem.push(State {
            c,
            out_1,
            out_2,
            last_set: usize::MAX,
        });
        HState {
            idx: self.mem.len() - 1,
        }
    }
    /// Takes the handle of a State object and returns a reference to the
    /// object.
    ///
    fn h2state(&self, hstate: HState) -> &State {
        &self.mem[hstate.idx]
    }
    /// Takes the handle of a State object and returns a mutable reference to
    /// the object.
    ///
    fn h2state_mut(&mut self, hstate: HState) -> &mut State {
        &mut self.mem[hstate.idx]
    }
    /// Takes a handle to the field of a state object and returns a mutable
    /// reference to that field.
    ///
    fn h2out_mut(&mut self, hout_n: HStateField) -> &mut Option<HState> {
        match hout_n.which {
            1 => &mut self.h2state_mut(hout_n.state).out_1,
            2 => &mut self.h2state_mut(hout_n.state).out_2,
            _ => {
                panic!("Invalid field id.")
            }
        }
    }
}

/// NFA fragment. These are used to assemble the State graph.
///
struct Frag {
    start: HState,
    out: StateList,
}
impl Frag {
    fn new(start: HState, out: StateList) -> Self {
        Frag { start, out }
    }
}

/// Represents transitions between states.
///
#[derive(PartialEq, Debug)]
enum Transition {
    Char(char),
    Split,
    Match,
}

/// A list of State field handles. StateList's are used together with `Frag`s
/// (fragments) when assembling the state graph. The name of this object may
/// seem like a slight misnomer; in the original implementation in C, this list
/// exists as a list of pointers to state pointers which enables updating the
/// out_1 and out_2 fields of the states.
///
struct StateList {
    list: Vec<HStateField>,
}
impl StateList {
    /// Create a new StateList. Not used for this problem.
    ///
    fn new() -> Self {
        Self { list: vec![] }
    }
    /// Creates new state list with a single state.out_n field handle.
    ///
    fn from_hout_n(hout_n: HStateField) -> Self {
        Self { list: vec![hout_n] }
    }
    /// Appends another StateList to this one. Not used for problem.
    ///
    fn append(&mut self, mut other: StateList) {
        self.list.append(&mut other.list);
    }
    /// Sets each State in the list to the one provided.
    ///
    fn patch(&self, s: HState, states: &mut States) {
        for &h in &self.list {
            *states.h2out_mut(h) = Some(s);
        }
    }
}
impl From<HStateField> for StateList {
    /// Convert a HStateField into a StateList with a single item.
    ///
    fn from(hout_n: HStateField) -> Self {
        Self::from_hout_n(hout_n)
    }
}

/// A set of states used during matching operations. Implemented as a vector
/// internally. The methods that take `set_id` use that variable to check
/// for set membership. This value increments each step operation.
///
struct FollowSet {
    set: Vec<HState>,
}
impl FollowSet {
    fn new() -> Self {
        FollowSet { set: vec![] }
    }
    /// Compute initial state set.
    ///
    fn start(&mut self, start: HState, states: &mut States, set_id: usize) {
        self.add(Some(start), states, set_id);
    }
    /// Returns the number of states.
    ///
    fn count(&self) -> usize {
        self.set.len()
    }

    fn clear(&mut self) {
        self.set.clear();
    }
    /// Add `state` to set, following "unlabeled arrows" (transitions).
    ///
    fn add(&mut self, state: Option<HState>, states: &mut States, set_id: usize) {
        use Transition::*;
        macro_rules! state {
            ($h:expr) => {
                states.h2state($h)
            };
        }

        if let Some(h) = state {
            if state!(h).last_set != set_id {
                states.h2state_mut(h).last_set = set_id;
                if state!(h).c == Split {
                    // Follow unlabeled transitions.
                    self.add(state!(h).out_1, states, set_id);
                    self.add(state!(h).out_2, states, set_id);
                } else {
                    self.set.push(h);
                }
            }
        }
    }
    /// Step the NFA from the states in this set past the character `c` to
    /// create next NFA state set `next_set`.
    ///
    fn step(&mut self, c: char, next_set: &mut FollowSet, states: &mut States, set_id: &mut usize) {
        use Transition::*;
        *set_id += 1;
        next_set.clear();
        for &h in &self.set {
            let s = states.h2state(h);
            if s.c == Char(c) || s.c == Char('.') {
                next_set.add(s.out_1, states, *set_id);
            }
        }
    }
    /// Determine if set contains a match.
    ///
    fn is_match(&self, states: &States) -> bool {
        self.set
            .iter()
            .any(|&h| states.h2state(h).c == Transition::Match)
    }
}

/// Convert regular expression to its postfix representation.
///
fn re2postfix(re: String) -> Vec<char> {
    let mut outputq = vec![];
    let mut opstack = vec![];
    let mut buf = Vec::with_capacity(2);
    let mut chars = re.chars();

    if let Some(ch) = chars.next() {
        outputq.push(ch);
    }
    for ch in chars {
        if ch != '*' {
            buf.push('\0');
        }

        buf.push(ch);

        for ch in buf.drain(..) {
            match ch {
                '*' => {
                    while opstack.last() == Some(&'*') {
                        outputq.push(opstack.pop().unwrap());
                    }
                    opstack.push(ch);
                }
                '\0' => {
                    while let Some(op) = opstack.pop() {
                        outputq.push(op);
                    }
                    opstack.push(ch);
                }
                _ => {
                    outputq.push(ch);
                }
            }
        }
    }
    while let Some(op) = opstack.pop() {
        outputq.push(op);
    }
    outputq
}

struct Solution {}

#[test]
fn test() {
    let suits = vec![("aa", "a", false), ("aa", "a*", true)];

    for (s, p, matches) in suits {
        assert_eq!(Solution::is_match(s.to_string(), p.to_string()), matches);
    }
}

fn main() {}
