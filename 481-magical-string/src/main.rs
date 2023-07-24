struct Solution;
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let n = n as usize;
        let mut i = 2;
        let mut s = vec![1, 2, 2];

        /* If next == 0, filling 1 to the array. If next == 1,
         * filling 2 to the array. This help us to remove one branch
         * with bit operation. */
        let mut next = 0;
        let mut total = 1;

        while s.len() < n {
            let repeat = s[i].min(n - s.len());

            for _ in 0..repeat {
                s.push(next + 1);
            }

            total += repeat * (1 - next);
            next ^= 1;
            i += 1;
        }

        total as i32
    }
}

fn main() {
    println!("{}", Solution::magical_string(10));
}
