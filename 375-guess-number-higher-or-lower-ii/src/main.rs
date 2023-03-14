struct Solution;
impl Solution {
    fn eval(dp: &Vec<Vec<i32>>, start: usize, end: usize) -> i32 {
        /* use every number in the range as root node. */
        let mut min = i32::MAX;
        for i in start..=end {
            let mut tmp = i as i32;
            /* If the left subtree exist and isn't only a single node */
            let mut cost1 = 0;
            if start < i - 1 {
                cost1 = dp[start][i - 1];
            }
            /* If the right subtree exist and isn't only a single node */
            let mut cost2 = 0;
            if i + 1 < end {
                cost2 = dp[i + 1][end];
            }

            tmp += cost1.max(cost2);
            min = min.min(tmp);
        }

        min
    }

    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        /* Let dp[i][j] denote the answer when guess between i ~ j. */
        let mut dp = vec![vec![0; n + 2]; n + 2];

        /* Initialization */
        dp[1][1] = 0;
        for i in 2..=n {
            dp[i][i] = i as i32;
        }

        for i in (1..n).rev() {
            let shift = n - i;
            for j in 1..=i {
                dp[j][j + shift] = Solution::eval(&dp, j, j + shift);
            }
        }

        dp[1][n] as i32
    }
}

fn main() {
    println!("{}", Solution::get_money_amount(3));
}
