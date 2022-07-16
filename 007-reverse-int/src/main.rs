struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let m;
        let max;
        if x < 0 {
            m = -1;
            x *= -1;
            max = 0x8000_0000 as u32;
        } else {
            m = 1;
            max = 0x7fff_ffff as u32;
        }

        let mut x = x as u32;
        let mut ans = 0;
        while x != 0 {
            let i = x % 10;
            if ans > (max - i) / 10 {
                return 0;
            } else {
                ans = ans * 10 + i;
            }
            x /= 10;
        }

        return ans as i32 * m;
    }
}

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
}
