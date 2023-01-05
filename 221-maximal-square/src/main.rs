struct Solution;
impl Solution {
    pub fn maximal_square(mut matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut dp = vec![0; n];
        let mut ans = 0;

        /* Scan the first row */
        for j in 0..n {
            dp[j] = matrix[0][j] as i32 - '0' as i32;
            ans = ans.max(dp[j]);
        }

        for i in 1..m {
            /* We don't store the left element's square size in
             * dp directly because we'll need the top left value later. */
            let mut prev = matrix[i][0] as i32 - '0' as i32;
            ans = ans.max(prev);
            for j in 1..n {
                let tmp;
                if matrix[i][j] == '1' {
                    /* the size of new square is decided by its left(prev),
                     * top(dp[j], and top left(dp[j -1]) element */
                    tmp = prev.min(dp[j - 1]).min(dp[j]) + 1;
                } else {
                    tmp = 0;
                }
                dp[j - 1] = prev;
                prev = tmp;
                ans = ans.max(prev);
            }
            dp[n - 1] = prev;
        }

        ans * ans
    }
}

fn main() {
    let v = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    println!("{}", Solution::maximal_square(v));
}
