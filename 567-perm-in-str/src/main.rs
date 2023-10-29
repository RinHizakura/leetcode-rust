struct Solution;
impl Solution {
    fn is_perm(c1: &Vec<usize>, c2: &Vec<usize>) -> bool {
        for i in 0..26 {
            if c1[i] != c2[i] {
                return false;
            }
        }

        return true;
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut code_1: u64 = 0;
        let mut code_2: u64 = 0;
        let len_1 = s1.len();
        let len_2 = s2.len();
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut count_1 = vec![0; 26];
        let mut count_2 = vec![0; 26];

        if len_1 > len_2 {
            return false;
        }

        for idx in 0..len_1 {
            code_1 += s1[idx] as u64;
            code_2 += s2[idx] as u64;

            count_1[(s1[idx] - b'a') as usize] += 1;
            count_2[(s2[idx] - b'a') as usize] += 1;
        }

        let mut start = 0;
        let mut end = len_1 - 1;

        // Simply adjust to match the while loop pattern
        code_2 -= s2[end] as u64;
        count_2[(s2[end] - b'a') as usize] -= 1;

        while end < len_2 {
            code_2 += s2[end] as u64;
            count_2[(s2[end] - b'a') as usize] += 1;

            if code_1 == code_2 {
                let check = Self::is_perm(&count_1, &count_2);
                if check == true {
                    return true;
                }
            }

            code_2 -= s2[start] as u64;
            count_2[(s2[start] - b'a') as usize] -= 1;
            start += 1;
            end += 1;
        }

        return false;
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string())
    );
}
