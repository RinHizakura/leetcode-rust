struct Solution;
impl Solution {
    fn go(nums: &Vec<i32>, idx: usize, len: i32, dp: &mut Vec<i32>) {
        if dp[idx] != 0 {
            return;
        }

        dp[idx] += len;
        Self::go(nums, nums[idx] as usize, len + 1, dp)
    }

    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dp = vec![0; n];

        for idx in 0..n {
            Self::go(&nums, idx, 1, &mut dp);
        }

        dp.into_iter().max().unwrap()
    }
}

fn main() {
    println!("{}", Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
}
