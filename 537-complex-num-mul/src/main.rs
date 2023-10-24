struct Solution;
impl Solution {
    fn parse(s: &[u8]) -> Vec<i32> {
        let mut idx = 0;

        let mut real_sign = 1;
        if s[idx] == b'-' {
            real_sign = -1;
            idx += 1;
        }

        let mut real: i32 = 0;
        while s[idx] != b'+' {
            real *= 10;
            real += (s[idx] - b'0') as i32;
            idx += 1;
        }

        idx += 1;

        let mut im_sign = 1;
        if s[idx] == b'-' {
            im_sign = -1;
            idx += 1;
        }

        let mut im: i32 = 0;
        while s[idx] != b'i' {
            im *= 10;
            im += (s[idx] - b'0') as i32;
            idx += 1;
        }

        vec![real * real_sign, im * im_sign]
    }

    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let v1 = Self::parse(num1.as_bytes());
        let v2 = Self::parse(num2.as_bytes());

        let real = v1[0] * v2[0] - v1[1] * v2[1];
        let im = v1[0] * v2[1] + v1[1] * v2[0];

        format!("{real}+{im}i")
    }
}

fn main() {
    println!(
        "{}",
        Solution::complex_number_multiply("2+-1i".to_string(), "1+1i".to_string())
    );
}
