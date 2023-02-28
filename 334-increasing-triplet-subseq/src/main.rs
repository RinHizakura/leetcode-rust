struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut left = i32::MAX;
        let mut mid = i32::MAX;

        for i in 0..len {
            /* When nums[i] > mid, a possible left will always be guaranteed(in the
             * next condition). */
            if nums[i] > mid {
                return true;
            } else if left < nums[i] && nums[i] <= mid {
                /* If left < nums[i] < mid, and if the triple (left, mid, ??) is valid,
                 * then the triple (left, nums[i], ??) should also be valid. So we can
                 * change mid to nums[i]. */
                mid = nums[i];
            } else {
                /* Otherwise, the situation should be nums[i] <= left. We can
                 * setup a new left value. */
                left = nums[i];
            }
        }

        false
    }
}

fn main() {
    println!("{}", Solution::increasing_triplet(vec![1, 3, 5]));
}
