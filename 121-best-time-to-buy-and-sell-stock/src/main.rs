struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut m = prices[0];
        let mut profit = 0;
        for i in 0..len {
            m = std::cmp::min(m, prices[i]);
            let dis = prices[i] - m;
            if dis > profit {
                profit = dis;
            }
        }
        profit
    }
}

fn main() {
    println!("Hello, world!");
}
