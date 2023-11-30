struct Solution;
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];

        // do nothing, and we will have one 'A'
        dp[1] = 0;

        for i in 1..=n {
            // copy 1 time + paste 1 time from current board
            let mut step = 1 + 1 + dp[i];
            let mut total = i + i;

            while total <= n {
                dp[total] = dp[total].min(step);

                // do paste again
                step += 1;
                total += i;
            }
        }

        dp[n as usize]
    }
}

fn main() {
    println!("{}", Solution::min_steps(10));
}
