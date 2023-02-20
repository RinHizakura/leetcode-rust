struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; (amount + 1) as usize];
        dp[0] = 0;

        for total in 1..=amount {
            for coin in &coins {
                if total >= *coin && dp[(total - *coin) as usize] != i32::MAX {
                    dp[total as usize] = dp[total as usize].min(1 + dp[(total - *coin) as usize]);
                }
            }
        }

        if dp[amount as usize] == i32::MAX {
            return -1;
        }
        dp[amount as usize]
    }
}

fn main() {
    println!("{}", Solution::coin_change(vec![1, 2, 5], 11));
}
