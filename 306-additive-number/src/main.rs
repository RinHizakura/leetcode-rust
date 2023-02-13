struct Solution;
impl Solution {
    pub fn get_num(num: &[u8], start: usize, end: usize) -> Option<usize> {
        let mut n: usize = 0;

        if num[start] == '0' as u8 && end > start {
            return None;
        }

        for idx in start..=end {
            n *= 10;
            n += (num[idx] - '0' as u8) as usize;
        }
        Some(n)
    }

    pub fn recursive(num: &[u8], n1: usize, n2: usize, start: usize, end: usize) -> bool {
        let mut ret = false;
        for k in start..=end {
            if let Some(n3) = Self::get_num(num, start, k) {
                if n1 + n2 == n3 {
                    if k == end {
                        return true;
                    } else {
                        ret = Self::recursive(num, n2, n3, k + 1, end);
                        if ret == true {
                            break;
                        }
                    }
                }
            }
        }

        ret
    }

    pub fn check_is_additive_number(num: &[u8], start: usize, end: usize) -> bool {
        for i in start..end {
            if let Some(n1) = Self::get_num(num, start, i) {
                for j in (i + 1)..end {
                    if let Some(n2) = Self::get_num(num, i + 1, j) {
                        if Self::recursive(num, n1, n2, j + 1, end) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_additive_number(num: String) -> bool {
        let num_bytes = num.as_bytes();
        // at least three numbers are required
        if num_bytes.len() < 3 {
            return false;
        }

        Self::check_is_additive_number(num_bytes, 0, num_bytes.len() - 1)
    }
}

fn main() {
    println!("{}", Solution::is_additive_number("112".to_string()));
}
