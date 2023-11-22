struct Solution;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;

        if c == 0 {
            return true;
        }

        let mut left = 0;
        let mut right = (c as f64).sqrt() as i64;

        while left <= right {
            let sum = left * left + right * right;
            if sum == c {
                return true;
            } else if sum > c {
                right -= 1;
            } else {
                left += 1;
            }
        }

        return false;
    }
}

fn main() {
    println!("{}", Solution::judge_square_sum(2));
}
