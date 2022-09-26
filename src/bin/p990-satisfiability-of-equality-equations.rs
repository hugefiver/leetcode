impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        use std::collections::{HashMap, HashSet};
        use std::{cell::RefCell, rc::Rc};

        let mut ne_pair = Vec::<(char, char)>::new();
        let mut eq_map: HashMap<char, Rc<RefCell<HashSet<char>>>> = HashMap::new();

        for s in equations {
            let s = s.chars().collect::<Vec<_>>();
            let a = s[0];
            let b = s[3];
            if s[1] == '=' {
                if a == b {
                    continue;
                }
                let in1 = eq_map.contains_key(&a);
                let in2 = eq_map.contains_key(&b);
                if in1 && in2 {
                    let set1 = eq_map.get(&a).unwrap().clone();
                    let set2 = eq_map.get(&b).unwrap();
                    if !Rc::ptr_eq(&set1, set2) {
                        set1.borrow_mut().extend(set2.borrow().iter());
                        for &c in set2.clone().borrow().iter() {
                            eq_map.insert(c, Rc::clone(&set1));
                        }
                    }
                } else if in1 {
                    let set = eq_map.get(&a).unwrap();
                    set.borrow_mut().insert(b);
                    eq_map.insert(b, Rc::clone(set));
                } else if in2 {
                    let set = eq_map.get(&b).unwrap();
                    set.borrow_mut().insert(a);
                    eq_map.insert(a, Rc::clone(set));
                } else {
                    let set = Rc::new(RefCell::new(HashSet::from([a, b])));
                    eq_map.insert(a, Rc::clone(&set));
                    eq_map.insert(b, Rc::clone(&set));
                }
            } else {
                if a == b {
                    return false;
                }
                ne_pair.push((a, b));
            }
        }

        for (a, b) in ne_pair.into_iter() {
            if eq_map
                .get(&a)
                .map_or(false, |set| set.borrow().contains(&b))
            {
                return false;
            }
        }

        true
    }
}

struct Solution;

fn main() {}
