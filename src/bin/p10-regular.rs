struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let re = RegularExp::parse(p).expect("cannot parse regular expression");
        re.match_all(s)
    }
}

struct RegularExp {
    matcher: Group,
}

impl RegularExp {
    pub fn parse(p: String) -> Option<RegularExp> {
        let mut idx = 0;
        let g = Self::parse_pattern(&p, &mut idx, '\0');
        match g {
            None => None,
            Some(g) => Some(RegularExp { matcher: g }),
        }
    }

    fn parse_pattern(s: &str, idx: &mut u32, until: char) -> Option<Group> {
        let mut g = Group { items: Vec::new() };

        while *idx < s.len() as u32 {
            let ch = s.as_bytes()[(*idx) as usize] as char;
            match ch {
                '(' => {
                    let group = Self::parse_group(s, idx);
                    if let Some(group) = group {
                        g.items.push(MatchItem::Group(group));
                    }
                }
                '.' => g.items.push(MatchItem::AnyChar),
                '*' => {
                    let last = g.items.pop().expect("'*' not belongs to a pattern");

                    let last_group = match last {
                        MatchItem::Group(g) => g,
                        _ => Group { items: vec![last] },
                    };

                    g.items.push(MatchItem::Star(last_group));
                }
                c if c == until => break,
                c => g.items.push(MatchItem::Char(c as u8)),
            }

            *idx += 1;
        }
        Some(g)
    }

    fn parse_group(s: &str, idx: &mut u32) -> Option<Group> {
        Self::parse_pattern(s, idx, ')')
    }

    pub fn match_all(self, s: String) -> bool {
        let s = s.as_bytes();

        if let Some(i) = Self::match_group(s, 0, &self.matcher) {
            i == s.len()
        } else {
            false
        }
    }

    fn match_items(s: &[u8], from: usize, m: &MatchItem) -> Option<usize> {
        use MatchItem::*;
        let i = from;

        if i >= s.len() {
            return match m {
                Star(_) => Some(i),
                Group(g) => {
                    let mut ret = Some(i);
                    for item in &g.items {
                        if Self::match_items(&s[i..], i, &item).is_none() {
                            ret = None;
                            break;
                        }
                    }
                    ret
                }
                _ => None,
            };
        }

        match m {
            AnyChar => Some(i + 1),
            &Char(ch) => {
                if s[i] == ch {
                    Some(i + 1)
                } else {
                    None
                }
            }
            Group(g) => Self::match_group(s, from, g),
            Star(g) => Self::match_star(s, from, g),
        }
    }

    fn match_group(s: &[u8], from: usize, g: &Group) -> Option<usize> {
        let mut ret = Some(from);
        let mut i = from;

        for item in &g.items {
            if let Some(next_idx) = Self::match_items(s, i, item) {
                i = next_idx;
            } else {
                ret = None;
                break;
            }
        }
        ret
    }

    fn match_star(s: &[u8], from: usize, g: &Group) -> Option<usize> {
        if let Some(next_idx) = Self::match_group(s, from, g) {
            if let Some(next_next_idx) = Self::match_star(s, next_idx, g) {
                Some(next_next_idx)
            } else {
                Some(next_idx)
            }
        } else {
            Some(from)
        }
    }
}

enum MatchItem {
    Char(u8),
    AnyChar,
    Group(Group),
    Star(Group),
}

struct Group {
    items: Vec<MatchItem>,
}

#[test]
fn test() {
    let suits = vec![("aa", "a", false), ("aa", "a*", true)];

    for (s, p, matches) in suits {
        assert_eq!(Solution::is_match(s.to_string(), p.to_string()), matches);
    }
}

fn main() {}
