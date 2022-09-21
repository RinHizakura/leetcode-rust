struct Solution;
impl Solution {
    pub fn step(s: &Vec<u8>, len: usize, i: usize, ans: &mut Vec<i32>) {
        if i >= len {
            return;
        }

        if s[i] != '0' as u8 {
            ans[i] += ans[i - 1];
        }

        if (s[i - 1] == '1' as u8)
            || (s[i - 1] == '2' as u8 && s[i] >= '0' as u8 && s[i] <= '6' as u8)
        {
            ans[i] += ans[i - 2];
        }

        Solution::step(s, len, i + 1, ans);
    }

    pub fn num_decodings(s: String) -> i32 {
        let s = s.into_bytes();
        let len = s.len();
        let mut ans = vec![0; len];

        if s[0] == '0' as u8 {
            return 0;
        }
        ans[0] = 1;

        if len > 1 {
            if s[1] != '0' as u8 {
                ans[1] += 1;
            }

            if (s[0] == '1' as u8) || (s[0] == '2' as u8 && s[1] >= '0' as u8 && s[1] <= '6' as u8)
            {
                ans[1] += 1;
            }
        }
        Solution::step(&s, len, 2, &mut ans);
        ans[len - 1]
    }
}

fn main() {
    println!("{}", Solution::num_decodings("226".to_string()));
}
