struct Solution;
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut ch_cnt = vec![0; 26];
        for c in s.chars() {
            ch_cnt[(c as u8 - b'a') as usize] += 1;
        }

        let mut num_cnt = vec![0; 10];

        // 'z' only appear for "zero"
        num_cnt[0] = ch_cnt[(b'z' - b'a') as usize];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[0];
        ch_cnt[(b'r' - b'a') as usize] -= num_cnt[0];
        ch_cnt[(b'o' - b'a') as usize] -= num_cnt[0];

        // 'w' only appear for "two"
        num_cnt[2] = ch_cnt[(b'w' - b'a') as usize];
        ch_cnt[(b't' - b'a') as usize] -= num_cnt[2];
        ch_cnt[(b'o' - b'a') as usize] -= num_cnt[2];

        // 'u' only appear for "four"
        num_cnt[4] = ch_cnt[(b'u' - b'a') as usize];
        ch_cnt[(b'f' - b'a') as usize] -= num_cnt[4];
        ch_cnt[(b'o' - b'a') as usize] -= num_cnt[4];
        ch_cnt[(b'r' - b'a') as usize] -= num_cnt[4];

        // 'x' only appear for "six"
        num_cnt[6] = ch_cnt[(b'x' - b'a') as usize];
        ch_cnt[(b's' - b'a') as usize] -= num_cnt[6];
        ch_cnt[(b'i' - b'a') as usize] -= num_cnt[6];

        // 'g' only appear for "eight"
        num_cnt[8] = ch_cnt[(b'g' - b'a') as usize];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[8];
        ch_cnt[(b'i' - b'a') as usize] -= num_cnt[8];
        ch_cnt[(b'h' - b'a') as usize] -= num_cnt[8];
        ch_cnt[(b't' - b'a') as usize] -= num_cnt[8];

        // The remaining 's' only appear for "seven"
        num_cnt[7] = ch_cnt[(b's' - b'a') as usize];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[7];
        ch_cnt[(b'v' - b'a') as usize] -= num_cnt[7];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[7];
        ch_cnt[(b'n' - b'a') as usize] -= num_cnt[7];

        // The remaining 'v' only appear for "five"
        num_cnt[5] = ch_cnt[(b'v' - b'a') as usize];
        ch_cnt[(b'f' - b'a') as usize] -= num_cnt[5];
        ch_cnt[(b'i' - b'a') as usize] -= num_cnt[5];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[5];

        // The remaining 'i' only appear for "nine"
        num_cnt[9] = ch_cnt[(b'i' - b'a') as usize];
        ch_cnt[(b'n' - b'a') as usize] -= num_cnt[9];
        ch_cnt[(b'n' - b'a') as usize] -= num_cnt[9];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[9];

        // The remaining 't' only appear for "three"
        num_cnt[3] = ch_cnt[(b't' - b'a') as usize];
        ch_cnt[(b'h' - b'a') as usize] -= num_cnt[3];
        ch_cnt[(b'r' - b'a') as usize] -= num_cnt[3];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[3];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[3];

        // The remaining characters are for "one"
        num_cnt[1] = ch_cnt[(b'o' - b'a') as usize];
        ch_cnt[(b'n' - b'a') as usize] -= num_cnt[1];
        ch_cnt[(b'e' - b'a') as usize] -= num_cnt[3];

        let zero = "0".to_string().repeat(num_cnt[0]);
        let one = "1".to_string().repeat(num_cnt[1]);
        let two = "2".to_string().repeat(num_cnt[2]);
        let three = "3".to_string().repeat(num_cnt[3]);
        let four = "4".to_string().repeat(num_cnt[4]);
        let five = "5".to_string().repeat(num_cnt[5]);
        let six = "6".to_string().repeat(num_cnt[6]);
        let seven = "7".to_string().repeat(num_cnt[7]);
        let eight = "8".to_string().repeat(num_cnt[8]);
        let nine = "9".to_string().repeat(num_cnt[9]);
        format!("{zero}{one}{two}{three}{four}{five}{six}{seven}{eight}{nine}")
    }
}

fn main() {
    println!("{}", Solution::original_digits("tow".to_string()));
}
