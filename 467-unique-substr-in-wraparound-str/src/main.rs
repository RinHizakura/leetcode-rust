struct Solution;
impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let len = s.len();
        let s = s.as_bytes();

        let mut dp: Vec<i32> = vec![0; 26];

        /* We can track the maximum length of valid wraparound string ended up with
        * different character. Then the total number of substring which ends
        * with that character can be got from it.

        * For example, if "zabc" is the longest wraparound substring ended with 'c', we
        * can have 4 substring "c", "bc", "abc", "zabc" from it. */
        let mut prev = (s[0] - b'a') as usize;
        dp[prev] = 1;

        let mut start = 0;
        for end in 1..len {
            let last = (s[end] - b'a') as usize;

            let dis = if last > prev {
                last - prev
            } else {
                26 + last - prev
            };

            if (dis != 1) {
                start = end;
            }

            dp[last] = dp[last].max((end - start + 1) as i32);

            prev = last;
        }

        dp.iter().sum()
    }
}

fn main() {
    println!("{}", Solution::new("cac".to_string()));
}
