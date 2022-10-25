struct Solution;
impl Solution {
    fn is_palindrome(s: &Vec<char>, mut start: usize, mut end: usize) -> bool {
        while start < end {
            if s[start] != s[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        return true;
    }

    fn recursive_solver(
        s: &Vec<char>,
        i: usize,
        len: usize,
        tmp: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if i >= len {
            ans.push(tmp.clone());
            return;
        }
        for idx in i..len {
            if Self::is_palindrome(s, i, idx) {
                tmp.push(s[i..idx + 1].iter().collect());
                Self::recursive_solver(s, idx + 1, len, tmp, ans);
                tmp.pop();
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let len = s.len();
        let mut tmp = vec![];
        let mut ans = vec![];
        Self::recursive_solver(&s.chars().collect(), 0, len, &mut tmp, &mut ans);
        ans
    }
}
fn main() {
    println!("{:?}", Solution::partition("aab".to_string()));
}
