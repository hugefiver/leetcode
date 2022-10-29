impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let n = plant_time.len();
        let mut plant_data: Vec<(i32, i32)> =
            plant_time.into_iter().zip(grow_time.into_iter()).collect();
        plant_data.sort_by(|a, b| b.1.cmp(&a.1));

        let mut free_time = 0;
        let mut max_bloom_time = 0;
        for i in 0..n {
            free_time = free_time + plant_data[i].0;
            max_bloom_time = max_bloom_time.max(free_time + plant_data[i].1);
        }

        max_bloom_time
    }
}

struct Solution;

fn main() {}
