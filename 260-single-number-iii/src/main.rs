struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        /* In the first pass, we got A ^ B */
        let mut a_xor_b = 0;
        for n in &nums {
            a_xor_b ^= n;
        }

        /* Let the rightmost bit of A^B being bit k, and
         * assume bit k of A is set. Under this assumption,
         * bit k of B should not be set. So we can exclude
         * any possible candidate of B in the vector to get A */

        /* It is impossible that set being zero, so we don't have
         * to handle such case */
        let set = a_xor_b.trailing_zeros();

        let mut a = 0;
        let mask = 1 << set;
        for n in &nums {
            if n & mask != 0 {
                a ^= n;
            }
        }

        vec![a, a ^ a_xor_b]
    }
}

fn main() {
    println!("{:?}", Solution::single_number(vec![2, 3, 3, 2, 4, 1]));
}
