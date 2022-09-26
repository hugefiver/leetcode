use std::collections::HashSet;

fn divisors(n: u32) -> u32 {
    let mut set = HashSet::new();
    get_divisors(n, &mut set);
    set.len() as u32
}

fn get_divisors(n: u32, set: &mut HashSet<u32>) {
    if n == 1 {
        set.insert(1);
        return;
    }
    for i in (1..=n/2).rev() {
        if n % i == 0 {
            let j = n / i;
            if !set.contains(&i) {
                set.insert(i);
                get_divisors(i, set);
            }
            if !set.contains(&j) {
                set.insert(j);
                get_divisors(j, set);
            }
        }
    }
}

#[test]
fn test() {
    for i in [1,2,4,5,12,25,4096] {
        let mut set = HashSet::new();
        get_divisors(i, &mut set);
        println!("{}: {:?}", i, set);
    }
}