use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let len = nums.len();
        let mut sum = 0;
        /* For the array [a1, a2, ... , an], map[r] = i means that
         * (a1 + a2 + ... + ai) % k == r.
         *
         * Let's say that (a1 + a2 + ... + ai) % k == r. If we find another subarray
         * that (a1 + a2 + ... + aj) % k == r, which j > i, than
         * (a1 + a2 + ... + aj) - (a1 + a2 + ... + ai) % k == r */
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..len {
            sum += nums[i];

            let r = sum % k;

            if i > 0 && r == 0 {
                return true;
            }
            if let Some(e) = map.get(&r) {
                if i as i32 - *e >= 2 {
                    return true;
                }
            }

            if nums[i] != 0 {
                map.insert(r, i as i32);
            }
        }

        return false;
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_subarray_sum(vec![1000000000], 1000000000)
    );
}
