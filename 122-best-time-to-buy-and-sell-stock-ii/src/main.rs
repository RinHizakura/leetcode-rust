struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();

        let mut profit = 0;
        for i in 1..len {
            let dis = prices[i] - prices[i - 1];
            if dis > 0 {
                profit += dis;
            }
        }
        profit
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![5, 4, 3, 2, 1]));
    println!("{}", Solution::max_profit(vec![1, 2, 3, 4, 5]));
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
}
