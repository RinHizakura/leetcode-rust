struct Solution;
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut left = 0;
        let mut right = len - 1;

        let mut prev = nums[left];
        while left + 1 < len && nums[left + 1] >= prev {
            left += 1;
            prev = nums[left];
        }
        // The array is sorted already
        if (left + 1) == len {
            return 0;
        }
        let mut prev = nums[right];
        while right - 1 > 0 && nums[right - 1] <= prev {
            right -= 1;
            prev = nums[right];
        }

        /* After the above process, we should find a candidate subarray by
         * relaxed restrictions. We have to follow the strict restrictions
         * as the problem stated however. */

        let mut left = left as i32;
        let mut right = right as i32;
        let len = len as i32;
        while left >= 0 && nums[left as usize] > nums[right as usize] {
            left -= 1;
        }
        while right < len && left >= 0 && nums[right as usize] < nums[left as usize] {
            right += 1;
        }

        let middle_min = *nums[(left + 1) as usize..right as usize]
            .iter()
            .min()
            .unwrap();
        let middle_max = *nums[(left + 1) as usize..right as usize]
            .iter()
            .max()
            .unwrap();

        while left >= 0 && nums[left as usize] > middle_min {
            left -= 1;
        }
        while right < len && nums[right as usize] < middle_max {
            right += 1;
        }
        (right - left - 1) as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_unsorted_subarray(vec![2, 100, 10, 1000])
    );
}
