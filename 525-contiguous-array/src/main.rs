use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut sum = 0;
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut ans = 0;

        for idx in 0..len {
            sum += (nums[idx] << 1) as i32 - 1;

            /* If we get two index with the same sum, there must has
             * same counts of zero and one between these indices. */
            if sum == 0 {
                ans = idx + 1;
            } else if let Some(i) = map.get(&sum) {
                ans = ans.max(idx - i);
            } else {
                map.insert(sum, idx);
            }
        }

        return ans as i32;
    }
}

fn main() {
    println!("{}", Solution::find_max_length(vec![1, 0, 1]));
}
