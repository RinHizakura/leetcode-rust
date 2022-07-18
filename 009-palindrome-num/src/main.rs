struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut i = x;
        let mut ans = 0;

        if i < 0 {
            return false;
        }

        while i != 0 {
            ans = ans * 10 + i % 10;
            i /= 10;
        }

        return x == ans;
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(121));
    println!("{}", Solution::is_palindrome(-121));
}
