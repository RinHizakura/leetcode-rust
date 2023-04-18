struct Solution;
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut step = 0;
        let mut n = n as i64;
        while n != 1 {
            if (n & 1) == 1 {
                /* If the number is odd, select the opertion which can
                 * generate most trailing zero. The exception is that
                 * the operation create an 2^n number. */
                let next_add = (n + 1).trailing_zeros();
                let next_sub = (n - 1).trailing_zeros();

                if (1 << next_sub) == (n - 1) {
                    n -= 1;
                } else if next_add > next_sub {
                    n += 1;
                } else {
                    n -= 1;
                }
                step += 1;
            } else {
                /* If the number is even, it has no choice to divide by 2 */
                let trailing_zero = n.trailing_zeros();
                n >>= trailing_zero;
                step += trailing_zero;
            }
        }
        step as i32
    }
}

fn main() {
    println!("{}", Solution::integer_replacement(4));
}
