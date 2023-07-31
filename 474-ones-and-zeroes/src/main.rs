struct Solution;
impl Solution {
    fn eval(s: &str) -> (i32, i32) {
        let mut ones = 0;

        for c in s.chars() {
            ones += (c as u8 - b'0') as i32;
        }

        let zeros = s.len() as i32 - ones;
        (zeros, ones)
    }

    fn find(
        v: &Vec<(i32, i32)>,
        idx: usize,
        m: i32,
        n: i32,
        dp: &mut Vec<Vec<Vec<Option<i32>>>>,
    ) -> i32 {
        if idx >= v.len() {
            return 0;
        }

        if let Some(r) = dp[idx][m as usize][n as usize] {
            return r;
        }

        let mut ans = Self::find(v, idx + 1, m, n, dp);

        if v[idx].0 <= m && v[idx].1 <= n {
            ans = ans.max(1 + Self::find(v, idx + 1, m - v[idx].0, n - v[idx].1, dp));
        }

        dp[idx][m as usize][n as usize] = Some(ans);

        return ans;
    }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let len = strs.len();
        let mut v = vec![(0, 0); len];

        for idx in 0..len {
            v[idx] = Self::eval(&strs[idx]);
        }

        let mut dp = vec![vec![vec![None; (n + 1) as usize]; (m + 1) as usize]; len];
        Self::find(&v, 0, m, n, &mut dp)
    }
}

fn main() {
    println!("{}", Solution::find_max_form(vec!["101".to_string()], 1, 1));
}
