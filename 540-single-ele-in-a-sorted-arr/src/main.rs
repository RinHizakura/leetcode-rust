struct Solution;
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut left = 0;
        let mut right = len - 1;

        while left < right && left < (len - 1) {
            let mid = (left + right) / 2;

            if mid & 1 == 1 {
                if nums[mid - 1] != nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] != nums[mid + 1] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }

        nums[left]
    }
}

fn main() {
    println!("{}", Solution::single_non_duplicate(vec![1, 1, 2, 2, 3]));
}
