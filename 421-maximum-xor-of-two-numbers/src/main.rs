use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        /* Let's start from the leftmost bit */
        let mut ans = 0;
        let mut mask = 0;

        for bit in (0..32).rev() {
            mask |= 1 << bit;

            let mut set = HashSet::new();
            for n in &nums {
                /* Insert every possible prefix for the binary
                 * representation of numbers in vector. */
                set.insert(*n & mask);
            }

            /* If a^ans = b, then a^b = ans */
            let candidate = ans | (1 << bit);
            for prefix in &set {
                if set.contains(&(prefix ^ candidate)) {
                    ans = candidate;
                    break;
                }
            }
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::find_maximum_xor(vec![1, 2, 3]));
}
