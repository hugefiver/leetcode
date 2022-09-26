use std::cmp::Reverse;

fn next_bigger_number(n: i64) -> i64 {
    let mut nums: Vec<i64> = vec![];
    let mut n = n;
    while n > 0 {
        nums.push(n % 10);
        n /= 10;
    }
    let mut success = false;
    for i in 1..nums.len() {
        let t = nums[i];
        if let Some((idx, _)) = nums[..i]
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x > t)
            .min_by_key(|&(_, x)| x)
        {
            (nums[i], nums[idx]) = (nums[idx], nums[i]);
            nums[..i].sort_by_key(|&x| Reverse(x));

            success = true;
            break;
        }
    }
    if success {
        nums.into_iter()
            .zip(0..)
            .fold(0, |n, (x, e)| n + x * (i64::pow(10, e)))
    } else {
        -1
    }
}
