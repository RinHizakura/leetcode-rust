struct Solution;
impl Solution {
    fn str_to_num(s: &str) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut num: i32 = 0;

        for i in 0..len {
            num *= 10;
            num += (s[i] - '0' as u8) as i32;
        }

        num
    }

    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<&str> = version1.split('.').collect();
        let v2: Vec<&str> = version2.split('.').collect();

        let l1 = v1.len();
        let l2 = v2.len();
        let mut cnt = 0;
        while cnt < l1 && cnt < l2 {
            let rev1 = Self::str_to_num(v1[cnt]);
            let rev2 = Self::str_to_num(v2[cnt]);

            if rev1 > rev2 {
                return 1;
            }
            if rev1 < rev2 {
                return -1;
            }
            cnt += 1;
        }

        if l1 > l2 {
            for i in cnt..l1 {
                let rev = Self::str_to_num(v1[i]);
                if rev != 0 {
                    return 1;
                }
            }
        }

        if l1 < l2 {
            for i in cnt..l2 {
                let rev = Self::str_to_num(v2[i]);
                if rev != 0 {
                    return -1;
                }
            }
        }

        0
    }
}

fn main() {
    println!(
        "{}",
        Solution::compare_version("1.0.1".to_string(), "1".to_string())
    );
}
