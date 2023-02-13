struct Solution;

// You don't have any stock, just get ready to buy one
const STATE_READY: usize = 0;
// You have one stock in hand, seeking the best time to sell it
const STATE_OWN: usize = 1;
// After selling the stock, cooldown.
const STATE_COOLDOWN: usize = 2;
const STATE_TOTAL: usize = 3;

impl Solution {
    fn update_state_ready(state: &mut Vec<Vec<i32>>, day: usize) {
        let today_idx = day & 1;
        let yesterday_idx = (day - 1) & 1;

        // From STATE_READY: do nothing
        let ans1 = state[yesterday_idx][STATE_READY];

        // From STATE_COOLDOWN
        let ans2 = state[yesterday_idx][STATE_COOLDOWN];

        state[today_idx][STATE_READY] = ans1.max(ans2);
    }

    fn update_state_own(state: &mut Vec<Vec<i32>>, day: usize, price: i32) {
        let today_idx = day & 1;
        let yesterday_idx = (day - 1) & 1;

        // From STATE_OWN: do nothing
        let ans1 = state[yesterday_idx][STATE_OWN];

        // From STATE_READY: buy the stock
        let ans2 = -price + state[yesterday_idx][STATE_READY];

        state[today_idx][STATE_OWN] = ans1.max(ans2);
    }

    fn update_state_cooldown(state: &mut Vec<Vec<i32>>, day: usize, price: i32) {
        let today_idx = day & 1;
        let yesterday_idx = (day - 1) & 1;

        // From STATE_OWN: sell the stock
        state[today_idx][STATE_COOLDOWN] = price + state[yesterday_idx][STATE_OWN];
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let total_day = prices.len();
        let mut state = vec![vec![i32::MIN; STATE_TOTAL]; 2];

        // Initialize for day 0
        state[0][STATE_READY] = 0;
        state[0][STATE_OWN] = -prices[0];

        for day in 1..total_day {
            for s in STATE_READY..STATE_TOTAL {
                match s {
                    STATE_READY => Self::update_state_ready(&mut state, day),
                    STATE_OWN => Self::update_state_own(&mut state, day, prices[day]),
                    STATE_COOLDOWN => Self::update_state_cooldown(&mut state, day, prices[day]),
                    _ => unreachable!(),
                }
            }
        }

        *state[(total_day - 1) & 1].iter().max().unwrap()
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![1,2,3,0,4]));
}
