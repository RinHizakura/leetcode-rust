struct Solution;
impl Solution {
    fn valid(s: &[u8], word: &String) -> bool {
        let mut idx = 0;
        let len = s.len();
        let mut total = 0;

        for c in word.chars() {
            while (idx < len) && (s[idx] != c as u8) {
                idx += 1;
            }

            if idx >= len {
                return false;
            }

            // find a matched character
            total += 1;
            idx += 1;
        }

        if total == word.len() {
            return true;
        }

        return false;
    }

    pub fn find_longest_word(s: String, mut dictionary: Vec<String>) -> String {
        // Sort according to the requirement
        dictionary.sort_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(&b)
            } else {
                b.len().cmp(&a.len())
            }
        });

        let s = s.as_bytes();

        for words in &dictionary {
            if Self::valid(s, words) {
                return words.to_owned();
            }
        }

        return "".to_string();
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_longest_word("abc".to_string(), vec!["abcd".to_string()])
    );
}
