struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        // cur means the last number in the longest sequence
        let mut cur_a = nums[0];
        let mut cur_b = nums[0];
        /* target == 0 means the next number should be smaller
         * than cur, otherwise it should be larger. */
        let mut target_a = 0;
        let mut target_b = 1;

        let mut len_a = 1;
        let mut len_b = 1;

        for n in nums {
            if (target_a == 0 && cur_a > n) || (target_a == 1 && cur_a < n) {
                len_a += 1;
                target_a ^= 1;
            }

            if (target_b == 0 && cur_b > n) || (target_b == 1 && cur_b < n) {
                len_b += 1;
                target_b ^= 1;
            }

            cur_a = n;
            cur_b = n;
        }

        len_a.max(len_b)
    }
}

fn main() {
    println!("{}", Solution::wiggle_max_length(vec![1,2,1]));
}
