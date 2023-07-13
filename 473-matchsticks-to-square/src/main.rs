struct Solution;
impl Solution {
    fn recursive(
        matchsticks: &Vec<i32>,
        n: usize,
        edge: i32,
        cur: i32,
        stage: i32,
        map: usize,
        dp: &mut Vec<bool>,
    ) -> bool {
        if stage == 4 {
            return true;
        }

        if dp[map] == false {
            return false;
        }

        for i in 0..n {
            if ((map >> i) & 1) != 0 {
                continue;
            }

            if cur + matchsticks[i] < edge {
                if Self::recursive(
                    matchsticks,
                    n,
                    edge,
                    cur + matchsticks[i],
                    stage,
                    map | (1 << i),
                    dp,
                ) {
                    return true;
                }
            } else if (cur + matchsticks[i]) == edge {
                if Self::recursive(matchsticks, n, edge, 0, stage + 1, map | (1 << i), dp) {
                    return true;
                }
            }
        }

        dp[map] = false;
        return false;
    }

    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let n = matchsticks.len();
        let mut dp = vec![true; 1 << n + 1];

        let total: i32 = matchsticks.iter().sum();
        let edge = total >> 2;
        if (edge << 2) != total {
            return false;
        }

        Self::recursive(&matchsticks, n, edge, 0, 0, 0, &mut dp)
    }
}

fn main() {
    println!("{}", Solution::makesquare(vec![1, 1, 2, 3]));
}
