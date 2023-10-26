use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut total = 0;
        let len = nums.len();

        /* A collection of sum including a0, (a0 + a1), ~ (a0 + a1 + ... + an), and
         * how many counts of them */
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut sum = 0;
        for idx in 0..len {
            sum += nums[idx];
            *map.entry(sum).or_default() += 1;

            if sum == k {
                total += 1;
            }
        }

        let mut sum = 0;
        for idx in 0..(len - 1) {
            sum += nums[idx];
            *map.get_mut(&sum).unwrap() -= 1;

            /* if we find (a0 + a1 + .. + ai) + k in the map, it means
             * (ai+1 + ai+2 + ... + aj) == k */
            let expect = k + sum;

            if let Some(cnt) = map.get(&expect) {
                total += *cnt;
            }
        }

        total
    }
}

fn main() {
    println!("{}", Solution::subarray_sum(vec![1, 2, 3], 5));
}
