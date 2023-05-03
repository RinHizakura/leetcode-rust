struct Solution;
impl Solution {
    fn recursive_solver(
        i: usize,
        len: usize,
        tmp: i32,
        ans: i32,
        nums: &Vec<i32>,
        dp: &mut Vec<Vec<i32>>,
    ) -> bool {
        if tmp > ans {
            return false;
        }

        if i == len {
            if tmp == ans {
                return true;
            } else {
                return false;
            }
        }

        if dp[i][tmp as usize] != -1 {
            return dp[i][tmp as usize] != 0;
        }

        let ans = Self::recursive_solver(i + 1, len, tmp + nums[i], ans, nums, dp)
            || Self::recursive_solver(i + 1, len, tmp, ans, nums, dp);

        dp[i][tmp as usize] = ans as i32;

        ans
    }

    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        let len = nums.len();

        /* It is impossible to partition if the sum is an odd number */
        if sum & 1 != 0 {
            return false;
        }

        /* If dp[i][j] == 1/0, it means if the total sum being j when
         * iterating at index i, the answer is true/false. Using
         * -1 as the uninitialized value. */
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; (sum / 2) as usize + 1]; len];

        Self::recursive_solver(0, len, 0, sum / 2, &nums, &mut dp)
    }
}

fn main() {
    println!("{}", Solution::can_partition(vec![1, 2, 3]));
}
