struct Solution;
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let n = nums.len();

        /* Choose from 0 to n, dp[i] means the number of largest subset when
         * the i'th number is choose */
        let mut dp = vec![1; n];
        /* prev[i] means the previous index of element which is in
         * the same subset. If prev[i] == -1, the i'th element in the
         * first member of the subset */
        let mut prev: Vec<i32> = vec![-1; n];

        let mut max = 0;
        let mut max_subset_idx: i32 = 0;
        for i in 0..n {
            for j in 0..i {
                if (nums[i] % nums[j] == 0) && (1 + dp[j] > dp[i]) {
                    dp[i] = 1 + dp[j];
                    prev[i] = j as i32;
                }
            }

            if dp[i] > max {
                max = dp[i];
                max_subset_idx = i as i32;
            }
        }

        let mut ans = vec![];
        let mut idx = max_subset_idx;
        while idx != -1 {
            ans.push(nums[idx as usize]);
            idx = prev[idx as usize];
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::largest_divisible_subset(vec![1, 2, 4, 8]));
}
