struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim();

        let s = s.as_bytes();
        let len = s.len();
        let mut i = 1;

        while s[len - i] != ' ' as u8 {
            i += 1;

            if i > len {
                break;
            }
        }

        (i - 1) as i32
    }
}

fn main() {
    println!("{}", Solution::length_of_last_word(" day".to_string()));
}
