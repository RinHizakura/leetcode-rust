use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut s = VecDeque::new();
        while column_number > 0 {
            let c = ('A' as u8 + ((column_number - 1) % 26) as u8) as char;
            column_number = (column_number - 1) / 26;

            s.push_front(c);
        }

        s.iter().collect()
    }
}

fn main() {
    println!("{}", Solution::convert_to_title(28));
}
