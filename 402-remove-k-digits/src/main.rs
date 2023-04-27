struct Solution;
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let num = num.as_bytes();
        let mut k = k as usize;

        let mut s = Vec::new();
        let mut l = 0;

        for n in num {
            while k > 0 && l > 0 && s[l - 1] > *n {
                s.pop();
                l -= 1;
                k -= 1;
            }

            /* Otherwise we put n to the String, but leading zero is
             * is ignored. */
            if *n != b'0' || l != 0 {
                s.push(*n);
                l += 1;
            }
        }

        /* If we still have quota */
        while k > 0 {
            s.pop();
            k -= 1;
        }

        if s.len() == 0 {
            return "0".to_string();
        }
        String::from_utf8(s).unwrap_or("0".to_string())
    }
}

fn main() {
    println!("{}", Solution::remove_kdigits("12345".to_string(), 1));
}
