struct Solution;
impl Solution {
    fn update_map(expected: &Vec<usize>, cur: &Vec<usize>, idx: usize, match_map: &mut usize) {
        if cur[idx] == expected[idx] {
            *match_map |= 1 << idx;
        } else {
            *match_map &= !(1 << idx);
        }
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let p_len = p.len();
        let s_len = s.len();

        let p = p.as_bytes();
        let s = s.as_bytes();

        let mut ans = vec![];
        if p_len > s_len {
            return ans;
        }

        let mut expected = vec![0; 26];
        let mut cur = vec![0; 26];
        let mut match_map = (1 << 27) - 1;

        for idx in 0..p_len {
            let idx0 = (p[idx] - b'a') as usize;
            let idx1 = (s[idx] - b'a') as usize;
            expected[idx0] += 1;
            cur[idx1] += 1;

            Self::update_map(&expected, &cur, idx0, &mut match_map);
            Self::update_map(&expected, &cur, idx1, &mut match_map);
        }

        if match_map == (1 << 27) - 1 {
            ans.push(0);
        }

        for idx in p_len..s_len {
            let idx0 = (s[idx - p_len] - b'a') as usize;
            let idx1 = (s[idx] - b'a') as usize;

            cur[idx0] -= 1;
            cur[idx1] += 1;

            Self::update_map(&expected, &cur, idx0, &mut match_map);
            Self::update_map(&expected, &cur, idx1, &mut match_map);

            if match_map == (1 << 27) - 1 {
                ans.push((idx - p_len + 1) as i32);
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_anagrams("abcabc".to_string(), "ba".to_string())
    );
}
