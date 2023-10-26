struct Solution;
impl Solution {
    fn go(mut n: i32, mask: i32, digits: &mut Vec<i32>) -> Result<i32, i32> {
        if mask > n {
            return Err(-1);
        }

        let d = (n / mask) % 10;
        n -= d * mask;

        let d = d as usize;
        digits[d] += 1;

        for i in (d + 1)..=9 {
            if digits[i] != 0 {
                digits[i] -= 1;

                let mut base = i as i32;

                for j in 0..=9 {
                    while digits[j] > 0 {
                        base = base.checked_mul(10).ok_or(-1)?;
                        base = base.checked_add(j as i32).ok_or(-1)?;
                        digits[j] -= 1;
                    }
                }

                return ((n / mask) * mask).checked_add(base as i32).ok_or(-1);
            }
        }

        if let Some(mask) = mask.checked_mul(10) {
            return Self::go(n, mask, digits);
        }

        Err(-1)
    }

    pub fn next_greater_element(mut n: i32) -> i32 {
        let mut digits = vec![0; 10];

        let d = n % 10;
        n -= d;
        digits[d as usize] += 1;

        Self::go(n, 10, &mut digits).unwrap_or(-1)
    }
}

fn main() {
    println!("{}", Solution::next_greater_element(132));
}
