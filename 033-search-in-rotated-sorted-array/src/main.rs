struct Solution;
impl Solution {
    pub fn recursive_solver(nums: Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
        let middle = (start + end) / 2;
        if nums[middle] == target {
            return middle as i32;
        }
        if nums[start] == target {
            return start as i32;
        }
        if nums[end] == target {
            return end as i32;
        }

        if start >= end || middle == 0 {
            return -1;
        }

        /* the left piece is in order */
        if nums[start] < nums[middle] {
            if target > nums[start] && target < nums[middle] {
                Solution::recursive_solver(nums, target, start, middle - 1)
            } else {
                Solution::recursive_solver(nums, target, middle + 1, end)
            }
        } else {
            if target > nums[middle] && target < nums[end] {
                Solution::recursive_solver(nums, target, middle + 1, end)
            } else {
                Solution::recursive_solver(nums, target, start, middle - 1)
            }
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        Solution::recursive_solver(nums, target, 0, len - 1)
    }
}

fn main() {
    println!("{}", Solution::search(vec![1, 3], 0));
}
