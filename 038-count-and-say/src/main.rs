struct Solution;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let s = Solution::count_and_say(n - 1);
        let s = s.into_bytes();
        let len = s.len();

        let mut ret_str = String::new();
        let mut prev = s[0];
        let mut count = 1;
        for i in 1..len {
            if prev == s[i] {
                count += 1;
            } else {
                ret_str.push((count + '0' as u8) as char);
                ret_str.push(prev as char);
                prev = s[i];
                count = 1;
            }
        }

        ret_str.push((count + '0' as u8) as char);
        ret_str.push(prev as char);

        ret_str
    }
}

fn main() {
    println!("{}", Solution::count_and_say(5));
    println!("{}", Solution::count_and_say(6));
    println!("{}", Solution::count_and_say(7));
}
