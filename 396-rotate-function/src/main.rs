struct Solution;
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        /* Assume the array being [a0, a1, a2, a3], then:
         * F(0) = 0 * a0 + 1 * a1 + 2 * a2 + 3 * a3
         * F(1) = (0 - 1 + 4) * a0 + (1 - 1) * a1 + (2 - 1) * a2 + (3 - 1) * a3
         * F(2) = (0 - 2 + 4) * a0 + (1 - 2 + 4) * a1 + (2 - 2) * a2 + (3 - 2) * a3
         */

        let len = nums.len();

        let mut f0 = 0;
        let mut sum = 0;
        for i in 0..len {
            f0 += i as i32 * nums[i];
            sum += nums[i];
        }

        let mut f_k = f0;
        let mut max = f0;
        for i in 1..len {
            f_k -= sum;
            f_k += len as i32 * nums[i - 1];

            max = max.max(f_k);
        }

        max
    }
}

fn main() {
    println!("{}", Solution::max_rotate_function(vec![1, 2, 3, 4]));
}
