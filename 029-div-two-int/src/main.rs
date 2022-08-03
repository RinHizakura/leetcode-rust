struct Solution;
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut sign = 1;

        let mut dividend_u: u32 = dividend as u32;
        let mut divisor_u: u32 = divisor as u32;

        if divisor < 0 {
            divisor_u = (divisor as i64 * -1) as u32;
            sign *= -1;
        }

        if dividend < 0 {
            dividend_u = (dividend as i64 * -1) as u32;
            sign *= -1;
        }

        while divisor_u & 1 == 0 && divisor_u != 0 {
            divisor_u >>= 1;
            dividend_u >>= 1;
        }

        let mut div = divisor_u;
        let mut accum = 1;
        let mut cnt:u32 = 0;
        let mut shift = 0;
        while dividend_u >= divisor_u {
            if dividend_u >= div {
                dividend_u -= div;
                cnt += accum;
                /* small optimization: take a large step if we success this time */
                if shift < 30 {
                    div <<= 1;
                    accum <<= 1;
                    shift += 1;
                }
            } else {
                if shift > 0 {
                    div >>= 1;
                    accum >>= 1;
                    shift -= 1;
                }
            }
        }

        if sign > 0 && cnt > (1 << 31) - 1 {
            ((1 << 31) as u32 - 1 ) as i32
        } else if sign < 0 && cnt > (1 << 31) {
            1 << 31
        } else {
            (cnt as u64 as i64 * sign) as i32
        }
    }
}

fn main() {
    println!("{}", Solution::divide(-2147483648, -1));
}
