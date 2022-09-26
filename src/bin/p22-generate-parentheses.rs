use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::foo(n, n).into_iter().collect()
    }

    fn foo(left: i32, right: i32) -> HashSet<String> {
        if left == 0 {
            HashSet::from_iter(vec![vec![')'; right as usize].into_iter().collect()])
        } else {
            let mut ret = HashSet::new();
            for s in Self::foo(left-1, right) {
                ret.insert(format!("({s}"));
            }

            if left < right {
                for s in Self::foo(left, right-1) {
                    ret.insert(format!("{s})"));
                    ret.insert(format!("){s}"));
                }
            }

            ret
        }
    }
}

impl Solution2 {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        fn generate(n: i32, left: i32, right: i32, p: String, result: &mut Vec<String>) {
            if right > left || left > n || right > n {
                return;
            }
            if left == n && right == n {
                result.push(p);
                return;
            }

            generate(n, left + 1, right, p.clone() + "(", result);
            generate(n, left, right + 1, p + ")", result);
        }

        generate(n, 0, 0, String::from(""), &mut result);

        result
    }

    pub fn generate_parenthesis_2(n: i32) -> Vec<String> {
        let (mut t, mut ans) = (vec!['(';2*n as usize], vec![]); 
        fn rec(a:usize, b:usize, t:&mut Vec<char>, ans:&mut Vec<String>) {
            if a+b==t.len() {ans.push(t.iter().collect::<String>());} else {
                if a<<1<t.len() {t[a+b]='('; rec(a+1,b,t,ans);}
                if b<a {t[a+b]=')'; rec(a,b+1,t,ans);}
            }
        }
        rec(0, 0, &mut t, &mut ans); ans
    }

    pub fn generate_parenthesis_3(n: i32) -> Vec<String> {
        let (mut t, mut ans) = (vec!['(';2*n as usize], vec![]); 
        fn rec(a:usize, b:usize, t:&mut Vec<char>, ans:&mut Vec<String>) {
            if a+b==t.len() {ans.push(t.iter().collect::<String>());} else {
                if a<<1<t.len() {t[a+b]='('; rec(a+1,b,t,ans);}
                if b<a {t[a+b]=')'; rec(a,b+1,t,ans);}
            }
        }
        rec(0, 0, &mut t, &mut ans); ans
    }
}

#[test]
fn test() {}

struct Solution;
struct Solution2;

fn main() {}
