struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut start = 0;
        let mut end = len - 1;

        while start <= end {
            let mid = (start + end + 1) / 2;

            if ((mid == 0) || (nums[mid] < nums[mid - 1]))
                && ((mid == len - 1) || (nums[mid] < nums[mid + 1]))
            {
                return nums[mid];
            }

            // if the right subarray follow the sequence
            if nums[mid] <= nums[end] {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        -1
    }
}

fn main() {
    println!("{}", Solution::find_min(vec![5, 1, 2, 3, 4]));
}
