use std::cmp::{max, min};

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // There are four step at more to perform: buy -> sell -> buy -> sell
        let len = prices.len();

        let mut dp = [[i32::MIN; 5]; 2];

        // initialize for day 0
        dp[0][0] = 0;
        dp[0][1] = -prices[0];

        // initialize for step 0
        dp[1][0] = 0;

        let mut max_ans = 0;
        for day in 1..len {
            let today_idx = day & 1;
            let yesterday_idx = (day + 1) & 1;
            for step in 1..=min(4, day + 1) {
                // do nothing
                let ans1 = dp[yesterday_idx][step];

                // do the next transaction
                let ans2;
                if step & 1 == 1 {
                    // buy
                    ans2 = -prices[day] + dp[yesterday_idx][step - 1];
                } else {
                    // sell
                    ans2 = prices[day] + dp[yesterday_idx][step - 1];
                }
                dp[today_idx][step] = max(ans1, ans2);
                if day == (len - 1) {
                    max_ans = max(dp[today_idx][step], max_ans);
                }
            }
        }

        max_ans
    }
}

fn main() {
    let v = vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0];
    println!("{}", Solution::max_profit(v));
}
