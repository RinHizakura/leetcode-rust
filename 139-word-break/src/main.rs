use std::collections::HashSet;
struct Solution;
impl Solution {
    fn recursive(s: &String, st: &HashSet<&str>, i: usize, len: usize, dp: &mut Vec<i32>) -> bool {
        if i == len {
            return true;
        }

        if dp[i] != -1 {
            return dp[i] != 0;
        }

        for idx in i..len {
            if st.contains(&s[i..idx + 1]) {
                let ret = Self::recursive(s, st, idx + 1, len, dp);
                if ret == true {
                    dp[idx + 1] = 1;
                    return true;
                }
            }
        }

        dp[i] = 0;
        return false;
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let len = s.len();
        let mut dp = vec![-1; len + 1];
        let st: HashSet<&str> = word_dict.iter().map(|s| s as &str).collect();
        Self::recursive(&s, &st, 0, len, &mut dp)
    }
}

fn main() {
    let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
    let dict = vec!["a".to_string(), "aa".to_string()];
    println!("{}", Solution::word_break(s, dict));
}
