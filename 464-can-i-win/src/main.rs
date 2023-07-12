struct Solution;
impl Solution {
    pub fn recursive(
        max_choosable_integer: i32,
        desired_total: i32,
        dp: &mut Vec<Option<bool>>,
        pool: u64,
    ) -> bool {
        if let Some(r) = dp[pool as usize] {
            return r;
        }

        if desired_total <= 0 {
            dp[pool as usize] = Some(false);
            return false;
        }

        for i in 1..=max_choosable_integer {
            if ((pool >> i) & 1) != 0 {
                continue;
            }

            if !Self::recursive(
                max_choosable_integer,
                desired_total - i,
                dp,
                pool | (1 << i),
            ) {
                dp[pool as usize] = Some(true);
                return true;
            }
        }

        dp[pool as usize] = Some(false);
        return false;
    }

    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let mut dp = vec![None; 1 << 20 + 1];

        let sum = max_choosable_integer * (max_choosable_integer + 1) / 2;
        if desired_total > sum {
            return false;
        }

        if desired_total == 0 {
            return true;
        }

        Self::recursive(max_choosable_integer, desired_total, &mut dp, 0)
    }
}

fn main() {
    println!("{}", Solution::can_i_win(1, 3));
}
