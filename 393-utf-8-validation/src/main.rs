struct Solution;
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let len = data.len();
        let mut i = 0;
        while i < len {
            if data[i] & 0x80 == 0 {
                /* 1 byte character */
                i += 1;
            } else if data[i] & 0xC0 != 0xC0 {
                return false;
            } else {
                let total;
                if data[i] & 0xE0 == 0xC0 {
                    total = 2;
                } else if data[i] & 0xF0 == 0xE0 {
                    total = 3;
                } else if data[i] & 0xF8 == 0xF0 {
                    total = 4;
                } else {
                    return false;
                }

                for idx in 1..total {
                    if (i + idx >= len) || (data[i + idx] & 0xC0 != 0x80) {
                        return false;
                    }
                }
                i += total;
            }
        }

        return true;
    }
}

fn main() {
    println!("{}", Solution::valid_utf8(vec![0xc0]));
}
