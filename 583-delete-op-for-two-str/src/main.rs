struct Solution;
impl Solution {
    fn lcs(word1: &[u8], word2: &[u8]) -> i32 {
        let l1 = word1.len();
        let l2 = word2.len();
        let mut dp = vec![vec![0; l2 + 1]; 2];

        for i in 1..(l1 + 1) {
            let idx = i & 1;
            let prev_idx = 1 - idx;
            for j in 1..(l2 + 1) {
                if word1[i - 1] == word2[j - 1] {
                    dp[idx][j] = dp[prev_idx][j - 1] + 1;
                } else {
                    dp[idx][j] = dp[prev_idx][j].max(dp[idx][j - 1]);
                }
            }
        }

        dp[l1 & 1][l2]
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let l1 = word1.len() as i32;
        let l2 = word2.len() as i32;
        let lcs = Self::lcs(word1.as_bytes(), word2.as_bytes());

        (l1 - lcs) + (l2 - lcs)
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_distance("a".to_string(), "ab".to_string())
    );
}
