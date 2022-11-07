struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut v = Vec::new();
        for token in s.trim().split_whitespace() {
            v.push(token);
            v.push(" ")
        }

        let s = v.into_iter().rev().collect::<String>();
        s.trim_start().to_string()
    }
}

fn main() {
    println!("{:?}", Solution::reverse_words("I am RinHizakura".to_string()));
}
