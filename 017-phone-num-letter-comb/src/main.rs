struct Solution;
impl Solution {
    pub fn recursive_solver(
        digits: &Vec<u8>,
        i: usize,
        len: usize,
        s: &mut String,
        ans: &mut Vec<String>,
    ) {
        if i == len {
            ans.push(s.clone());
            return;
        }

        let l = if digits[i] == 'p' as u8 || digits[i] == 'w' as u8 {
            4
        } else {
            3
        };

        for j in 0..l {
            s.push((digits[i] + j as u8) as char);
            Solution::recursive_solver(digits, i + 1, len, s, ans);
            s.pop();
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let len = digits.len();
        let mut digits = digits.into_bytes();
        let mut ans = Vec::new();

        if len == 0 {
            return ans;
        }

        for i in 0..len {
            let c = digits[i];
            if c < '8' as u8 {
                digits[i] = c + 47 + (c - '2' as u8) * 2
            } else {
                digits[i] = c + 60 + (c - '8' as u8) * 2;
            }
        }

        let mut s = String::new();
        Solution::recursive_solver(&digits, 0, len, &mut s, &mut ans);

        ans
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("234".to_string()));
}
