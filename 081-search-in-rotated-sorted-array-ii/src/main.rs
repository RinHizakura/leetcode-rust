struct Solution;
impl Solution {
    pub fn recursive_solver(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> bool {
        let middle = (start + end) / 2;
        if nums[middle] == target || nums[start] == target || nums[end] == target {
            return true;
        }

        if start >= end || middle == 0 {
            return false;
        }

        if nums[start] < nums[middle] {
            /* the left piece is in order */
            if target > nums[start] && target < nums[middle] {
                Solution::recursive_solver(nums, target, start, middle - 1)
            } else {
                Solution::recursive_solver(nums, target, middle + 1, end)
            }
        } else if nums[start] > nums[middle] {
            /* the right piece is in order */
            if target > nums[middle] && target < nums[end] {
                Solution::recursive_solver(nums, target, middle + 1, end)
            } else {
                Solution::recursive_solver(nums, target, start, middle - 1)
            }
        } else {
            /* not sure the order, find both direction */
            let found = Solution::recursive_solver(nums, target, middle + 1, end);
            if found == true {
                return true;
            }
            Solution::recursive_solver(nums, target, start, middle - 1)
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let len = nums.len();
        Solution::recursive_solver(&nums, target, 0, len - 1)
    }
}

fn main() {
    //           0 1 2 3 4 5 6 7 8 9 0 1 2 3
    let v = vec![1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1];
    println!("{}", Solution::search(v, 2));
}
