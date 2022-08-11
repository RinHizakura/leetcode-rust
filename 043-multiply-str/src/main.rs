/* yuck...... */
struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let zero_str = "0".to_string();
        if num1 == zero_str || num2 == zero_str {
            return zero_str;
        }

        let num1 = num1.into_bytes();
        let num2 = num2.into_bytes();
        let len1 = num1.len();
        let len2 = num2.len();

        let mut s = vec![0; len1 + len2];

        for i in (0..len1).rev() {
            let i1 = num1[i] - '0' as u8;
            for j in (0..len2).rev() {
                let i2 = num2[j] - '0' as u8;

                let tmp = i1 * i2 + s[i + j + 1];
                let mut carry = tmp / 10;
                let mut cnt = 1;
                s[i + j + 1] = tmp % 10;

                while carry != 0 {
                    let tmp = carry + s[i + j + 1 - cnt];
                    carry = tmp / 10;
                    s[i + j + 1 - cnt] = tmp % 10;
                    cnt += 1;
                }
            }
        }

        let mut idx = 0;
        while s[idx] == 0 && idx < len1 + len2 {
            idx += 1;
        }
        let first = idx;

        while idx < len1 + len2 {
            s[idx] += '0' as u8;
            idx += 1;
        }

        String::from_utf8(s[first..].to_vec()).unwrap()
    }
}

fn main() {
    println!("{}", Solution::multiply("2".to_string(), "3".to_string()));
}
