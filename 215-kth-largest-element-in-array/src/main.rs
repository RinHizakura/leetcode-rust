struct Solution;
impl Solution {
    fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        let pivot = nums[right];

        let mut store = left;
        for i in left..right {
            if nums[i] > pivot {
                nums.swap(i, store);
                store += 1;
            }
        }

        nums.swap(store, right);
        store
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        // Change to 0-based order
        let k = k as usize - 1;
        let len = nums.len();
        /* TODO: we can optimize the selection of pivot
         * by median of medians */

        let mut left = 0;
        let mut right = len - 1;

        let mut pivot_idx;
        loop {
            pivot_idx = Self::partition(&mut nums, left, right);
            if pivot_idx == k {
                break;
            }

            if pivot_idx > k {
                right = pivot_idx - 1;
            } else {
                left = pivot_idx + 1
            }
        }

        nums[pivot_idx]
    }
}

fn main() {
    println!("{}", Solution::find_kth_largest(vec![2, 3, 4, 1], 2));
}
