struct Solution;
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        /* The following numbers are used to describe the array */
        let mut head = 1;
        let mut remaining = n;
        let mut delta = 1;
        /* dir 0 means from left to right, dir 1 means from right to left */
        let mut dir = 0;

        while remaining != 1 {
            if (dir == 0) || ((dir == 1) && (remaining & 1 == 1)) {
                head += delta;
            }

            /* Flip the direction */
            dir ^= 1;
            /* In each round, the remaining number will reduce by a half */
            remaining >>= 1;
            delta <<= 1;
        }

        head
    }
}

fn main() {
    println!("{}", Solution::last_remaining(9));
}
