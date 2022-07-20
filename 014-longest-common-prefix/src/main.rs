struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let len = strs.len();
        let mut s1 = strs[0].clone();
        if len == 1 {
            return s1;
        }

        for i in 1..len {
            let mut cnt = 0;
            let s2 = &strs[i];
            let mut c1_iter = s1.chars();
            let mut c2_iter = s2.chars();

            while let (Some(c1), Some(c2)) = (c1_iter.next(), c2_iter.next()) {
                if c1 != c2 {
                    break;
                }

                cnt += 1;
            }

            s1.truncate(cnt);
        }

        s1
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string(), "a".to_string(),])
    );
}
