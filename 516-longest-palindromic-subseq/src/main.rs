struct Solution;
impl Solution {
    fn recursive_solver(s: &[u8], len: usize, start: i32, end: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if start as usize >= len || end < 0 {
            return 0;
        }

        let s_idx = start as usize;
        let e_idx = end as usize;

        if dp[s_idx][e_idx] != -1 {
            return dp[s_idx][e_idx];
        }

        if s_idx > e_idx {
            dp[s_idx][e_idx] = 0;
        } else if s_idx == e_idx {
            dp[s_idx][e_idx] = 1;
        } else if s[s_idx] == s[e_idx] {
            dp[s_idx][e_idx] = 2 + Self::recursive_solver(s, len, start + 1, end - 1, dp);
        } else {
            let a = Self::recursive_solver(s, len, start, end - 1, dp);
            let b = Self::recursive_solver(s, len, start + 1, end, dp);
            dp[s_idx][e_idx] = a.max(b);
        }

        return dp[s_idx][e_idx];
    }

    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut dp = vec![vec![-1; len]; len];

        Self::recursive_solver(s, len, 0, len as i32 - 1, &mut dp);

        dp[0][len - 1]
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_palindrome_subseq("abba".to_string())
    );
}
