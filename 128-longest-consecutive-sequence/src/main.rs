use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let s: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;

        for n in &s {
            if s.contains(&(n - 1)) {
                continue;
            }

            let mut cnt = 1;
            while s.contains(&(n + cnt)) {
                cnt += 1;
            }
            ans = ans.max(cnt);
        }

        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2])
    );
}
