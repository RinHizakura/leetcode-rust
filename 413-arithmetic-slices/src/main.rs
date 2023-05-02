struct Solution;
impl Solution {
    pub fn number_of_arithmetic_slices(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len < 3 {
            return 0;
        }

        // Convert the array to the array of difference
        for idx in 0..(len - 1) {
            nums[idx] = nums[idx + 1] - nums[idx];
        }

        let mut sum = 0;
        let mut tmp = 0;
        let mut diff = i32::MAX;
        /* We can count how many continuous difference are the same first.
         * For a new same difference we found, consider the following example
         * to see how can we evalute the answer:
         * [1, 2, 3] -> 1
         * [1, 2, 3, 4] -> 1 + 2
         * [1, 2, 3, 4, 5] -> 1 + 2 + 3
         */
        for idx in 0..(len - 1) {
            if diff == nums[idx] {
                tmp += 1;
            } else {
                tmp = 0;
            }

            diff = nums[idx];
            sum += tmp;
        }

        sum
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", Solution::number_of_arithmetic_slices(v));
}
