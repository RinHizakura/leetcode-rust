struct Solution;
impl Solution {
    fn eval(s: &[u8], start: usize, end: usize, len: usize) -> i32 {
        let mut ans = 0;
        let mut sz = 0;

        while start >= sz && end + sz < len && s[start - sz] == s[end + sz] {
            ans += 1;
            sz += 1;
        }

        return ans;
    }

    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();

        let mut ans = 0;
        for i in 0..len {
            ans += Self::eval(s, i, i, len);
            ans += Self::eval(s, i, i + 1, len);
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::count_substrings("aaa".to_owned()));
}
