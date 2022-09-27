struct Solution;
impl Solution {
    fn valid(v: &[u8], len: usize) -> bool {
        if len == 0 || (len > 1 && v[0] == '0' as u8) {
            return false;
        }

        let mut num: usize = 0;
        for i in 0..len {
            num *= 10;
            num += (v[i] - '0' as u8) as usize;
            if num > 255 {
                return false;
            }
        }

        return true;
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let len = s.len();
        let s = s.into_bytes();
        let mut ans = vec![];

        if len < 4 {
            return ans;
        }

        for i in 2..(len - 1) {
            let mut v1;
            let mut v2;
            let mut v3;
            let mut v4;
            for j in 1..i {
                v1 = Solution::valid(&s[0..j], j);
                v2 = Solution::valid(&s[j..i], i - j);
                for k in i..len {
                    v3 = Solution::valid(&s[i..k], k - i);
                    v4 = Solution::valid(&s[k..len], len - k);

                    if v1 && v2 && v3 && v4 {
                        let tmp = String::from("");
                        let tmp = tmp
                            + std::str::from_utf8(&s[0..j]).unwrap()
                            + "."
                            + std::str::from_utf8(&s[j..i]).unwrap()
                            + "."
                            + std::str::from_utf8(&s[i..k]).unwrap()
                            + "."
                            + std::str::from_utf8(&s[k..len]).unwrap();
                        ans.push(tmp);
                    }
                }
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::restore_ip_addresses("25525511135".to_string())
    );
    println!("{:?}", Solution::restore_ip_addresses("0000".to_string()));
}
