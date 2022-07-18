struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim().to_string();
        let len = s.len();
        let s = s.into_bytes();
        let mut ans: u32 = 0;
        let mut start = 0;
        let mut sign: i32 = 1;

        if len == 0 {
            return 0;
        }

        if s[0] == 45 {
            sign = -1;
            start = 1;
        } else if s[0] == 43 {
            sign = 1;
            start = 1;
        }

        for i in start..len {
            if s[i] < 48 || s[i] > 57 {
                break;
            }

            if sign == 1 && ans > (i32::MAX as u32 / 10) {
                return i32::MAX;
            } else if sign == -1 && ans > ((i32::MAX as u32 + 1) / 10) {
                return i32::MIN;
            }
            ans = ans * 10;

            let k = (s[i] as u32 - 48);
            if sign == 1 && ans > i32::MAX as u32 - k {
                return i32::MAX;
            } else if sign == -1 && ans > ((i32::MAX as u32 + 1) - k) {
                return i32::MIN;
            }
            ans += k;
        }

        return ans as i32 * sign;
    }
}

fn main() {
    println!("{}", Solution::my_atoi("243".to_string()));
    println!("{}", Solution::my_atoi("+243".to_string()));
    println!("{}", Solution::my_atoi("-243".to_string()));
    println!("{}", Solution::my_atoi("   243".to_string()));
}
