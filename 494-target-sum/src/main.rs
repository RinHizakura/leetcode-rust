struct Solution;
impl Solution {
    fn go(
        nums: &Vec<i32>,
        idx: usize,
        len: usize,
        target: i32,
        sum: i32,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if idx == len {
            if target == sum {
                return 1;
            }
            return 0;
        }

        if dp[idx][(sum + 20000) as usize] != -1 {
            return dp[idx][(sum + 20000) as usize];
        }

        let cur = nums[idx];

        let mut sol = Self::go(nums, idx + 1, len, target, sum + cur, dp);
        sol += Self::go(nums, idx + 1, len, target, sum - cur, dp);

        dp[idx][(sum + 20000) as usize] = sol;

        return sol;
    }

    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut dp = vec![vec![-1; 40000]; len];
        Self::go(&nums, 0, nums.len(), target, 0, &mut dp)
    }
}

fn main() {
    println!("{}", Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}
