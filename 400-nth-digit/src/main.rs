struct Solution;
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        /* Take this as special case directly */
        if n < 9 {
            return n;
        }

        let n = n as i64;
        let mut base: i64 = 1;
        let mut total: i64 = 9;
        let mut digit: i64 = 1;

        let mut prev_accum = 0;
        let mut accum = total * digit;

        while n > accum {
            prev_accum = accum;

            total *= 10;
            base *= 10;
            digit += 1;
            accum += total * digit;
        }

        let n = n - prev_accum - 1;
        let target_num = base + n / digit;
        let pos = (digit - 1) - (n % digit);

        (target_num / 10_u32.pow(pos as u32) as i64 % 10) as i32
    }
}

fn main() {
    println!("{}", Solution::find_nth_digit(5));
}
