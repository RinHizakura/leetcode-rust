struct Solution;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];

        dp[1] = 1;

        let mut k = 2;
        let mut square = k * k;

        for i in 2..n + 1 {
            if i == square {
                k += 1;
                square = k * k;
                dp[i as usize] = 1;
                continue;
            }

            let mut m = i32::MAX;
            for j in 1..k {
                let tmp = 1 + dp[(i - (j * j)) as usize];
                if m > tmp {
                    m = tmp;
                }
            }
            dp[i as usize] = m;
        }

        dp[n as usize]
    }
}

fn main() {
    println!("{}", Solution::num_squares(12));
}
