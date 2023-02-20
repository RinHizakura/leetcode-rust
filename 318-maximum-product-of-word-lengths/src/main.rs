struct Solution;
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let len = words.len();
        let mut bitmask = vec![0; len];

        let mut max = 0;
        for i in 0..len {
            let bytes = words[i].as_bytes();
            for byte in bytes {
                bitmask[i] |= 1 << (byte - b'a');
            }

            for j in 0..i {
                if bitmask[i] & bitmask[j] == 0 {
                    let tmp = words[i].len() * words[j].len();
                    max = max.max(tmp);
                }
            }
        }

        max as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_product(vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "xtfn".to_string()
        ])
    );
}
