struct Solution;
impl Solution {
    fn is_divisible(a: usize, b: usize) -> bool {
        (a / b * b) == a
    }

    fn go(mask: u32, idx: usize, n: usize) -> i32 {
        if idx > n {
            return 1;
        }

        let mut sum = 0;

        for i in 1..idx {
            if (((mask >> i) & 1) == 0) && Self::is_divisible(idx, i) {
                sum += Self::go(mask | (1 << i), idx + 1, n);
            }
        }

        let mut i = idx;

        while i <= n {
            if ((mask >> i) & 1) == 0 {
                sum += Self::go(mask | (1 << i), idx + 1, n);
            }
            i += idx;
        }

        return sum;
    }

    pub fn count_arrangement(n: i32) -> i32 {
        return Self::go(0, 1, n as usize);
    }
}

fn main() {
    println!("{}", Solution::count_arrangement(15));
}
