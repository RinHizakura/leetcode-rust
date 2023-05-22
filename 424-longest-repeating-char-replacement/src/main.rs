struct Solution;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let len = s.len();

        let mut left = 0;
        let mut right = 0;
        let s = s.as_bytes();

        let mut max = 0;
        let mut freq = vec![0; 26];
        let mut freq_max = 0;
        let mut new_right = true;

        while right < len {
            if new_right {
                freq[(s[right] - b'A') as usize] += 1;
                freq_max = freq_max.max(freq[(s[right] - b'A') as usize]);
            } else {
                freq[(s[left - 1] - b'A') as usize] -= 1;
                /* FIXME: This might waste time. Can we replace it
                 * with some O(1) approach? */
                freq_max = *freq.iter().max().unwrap();
            }

            let window_size = right - left + 1;
            let cost = window_size - freq_max;

            if cost as i32 <= k {
                max = max.max(window_size);
                right += 1;
                new_right = true;
            } else {
                left += 1;
                new_right = false;
            }
        }

        max as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::character_replacement("AAABBBCC".to_string(), 3)
    );
}
