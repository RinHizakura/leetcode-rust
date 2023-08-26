struct Solution;
impl Solution {
    fn pick(nums: &Vec<i32>, left: i32, right: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if left == right {
            return nums[left as usize];
        }

        if left > right {
            return 0;
        }

        if dp[left as usize][right as usize] != -1 {
            return dp[left as usize][right as usize];
        }

        let next_1 = Self::pick(nums, left + 2, right, dp);
        let next_2 = Self::pick(nums, left + 1, right - 1, dp);
        let next_3 = Self::pick(nums, left, right - 2, dp);

        /* case1: p1 pick left. Because p2 will also play optimal, p1 can only select
         * the minimal answer between after p2 choosing. */
        let t1 = nums[left as usize] + next_1.min(next_2);
        // case2: p1 pick right
        let t2 = nums[right as usize] + next_3.min(next_2);

        let ans = t1.max(t2);
        dp[left as usize][right as usize] = ans;
        return ans;
    }

    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let len = nums.len();

        let left = 0;
        let right = len - 1;
        let mut dp = vec![vec![-1; len]; len];
        let score = Self::pick(&nums, left as i32, right as i32, &mut dp);

        let sum: i32 = nums.iter().sum();
        score >= (sum - score)
    }
}

fn main() {
    println!("{}", Solution::predict_the_winner(vec![4, 4, 3, 1]));
}
