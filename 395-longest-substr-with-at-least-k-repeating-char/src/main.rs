struct Solution;
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let len = s.len();

        let mut max = 0;
        /* We use the numbers of the unique charater in the
         * sliding windows as constraint. */
        for unique_c_num in 1..26 {
            let mut left = 0;
            let mut diff_cnt = 0;
            let mut repeat_k_cnt = 0;
            let mut freq = [0; 26];

            for right in 0..len {
                let c = s[right];
                /* Extend the slinding window to contain this char */
                if freq[(c - b'a') as usize] == 0 {
                    diff_cnt += 1;
                }
                freq[(c - b'a') as usize] += 1;

                if freq[(c - b'a') as usize] == k {
                    repeat_k_cnt += 1;
                }

                /* Shrink from the left to meet the constraint */
                while diff_cnt > unique_c_num {
                    let c = s[left];
                    if freq[(c - b'a') as usize] == k {
                        repeat_k_cnt -= 1;
                    }
                    freq[(c - b'a') as usize] -= 1;

                    if freq[(c - b'a') as usize] == 0 {
                        diff_cnt -= 1;
                    }
                    left += 1;
                }

                if repeat_k_cnt == diff_cnt {
                    max = max.max(right - left + 1);
                }
            }
        }
        max as i32
    }
}

fn main() {
    println!("{}", Solution::longest_substring("aaabb".to_string(), 3));
}
