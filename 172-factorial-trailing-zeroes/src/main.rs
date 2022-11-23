struct Solution;
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut cnt = 0;
        let mut div = 5;
        for _ in 1..10 {
            let tmp = n / div;
            if tmp == 0 {
                break;
            }
            cnt += tmp;
            div *= 5;
        }
        cnt
    }
}

fn main() {
    println!("{}", Solution::trailing_zeroes(5));
}
