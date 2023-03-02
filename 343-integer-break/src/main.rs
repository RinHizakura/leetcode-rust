struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];

        dp[1] = 1;
        dp[2] = 1;

        for i in 3..=n {
            let mut max = 0;
            /* We try to break the number into
             *     1. two (j * (i-j))
             *     2. one plus a number which will be brokedown (dp[j] * (i-j))
             * Then we can check which is the maximum between them.
             */
            for j in 1..i {
                let tmp = j.max(dp[j]) * (i - j);
                max = tmp.max(max);
            }
            dp[i] = max;
        }

        dp[n] as i32
    }
}

fn main() {
    println!("{}", Solution::integer_break(3));
}
