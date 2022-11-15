struct Solution;
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut start = 0;
        let mut end = len - 1;

        while start <= end {
            let mid = (start + end) / 2;

            if (mid <= 0 || nums[mid] > nums[mid - 1])
                && (mid >= len - 1 || nums[mid] > nums[mid + 1])
            {
                return mid as i32;
            }

            if mid > 0 && nums[mid] < nums[mid - 1] {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        unreachable!()
    }
}

fn main() {
    println!("{}", Solution::find_peak_element(vec![1,3,1,2]));
}
