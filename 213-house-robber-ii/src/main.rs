struct Solution;
impl Solution {
    fn __rob(nums: &[i32], dp: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        dp[0] = nums[0];
        dp[1] = nums[1];
        dp[2] = nums[2] + dp[0];

        for i in 3..len {
            dp[i & 0x3] = nums[i] + dp[(i - 2) & 0x3].max(dp[(i - 3) & 0x3]);
        }

        dp[(len - 1) & 0x3].max(dp[(len - 2) & 0x3])
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len <= 3 {
            return *nums.iter().max().unwrap();
        }

        let mut dp = vec![0; 4];
        // The first one is guaranteed to be skipped
        let a1 = Self::__rob(&nums[1..len], &mut dp);

        // The last one is guaranteed to be skipped
        let a2 = Self::__rob(&nums[0..len - 1], &mut dp);

        a1.max(a2)
    }
}

fn main() {
    println!("{}", Solution::rob(vec![1, 3, 3, 100]));
}
