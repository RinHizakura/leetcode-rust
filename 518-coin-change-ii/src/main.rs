struct Solution;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; (amount + 1) as usize];

        dp[0] = 1;
        for coin in coins {
            for sum in 1..=amount {
                if sum >= coin {
                    dp[sum as usize] += dp[(sum - coin) as usize];
                }
            }
        }

        return dp[amount as usize];
    }
}

fn main() {
    println!("{}", Solution::change(5, vec![1, 2, 5]));
}
