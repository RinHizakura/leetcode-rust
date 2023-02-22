struct Solution;

#[macro_export]
macro_rules! map_idx {
    ($idx:expr, $n:expr) => {
        {
            if $idx < $n / 2 {
                ($idx << 1) + 1
            } else {
                ($idx - ($n >> 1)) << 1
            }
        }
    };
}

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

    /* reference: leetcode 215
     * NOTE: This implementation will change the arrangement of element in
     * the input vector. We allow this because it doesn't matter for our
     * problem here. */
    pub fn find_kth_largest(nums: &mut Vec<i32>, k: usize) -> i32 {
        let len = nums.len();
        /* TODO: we can optimize the selection of pivot
         * by median of medians */

        let mut left = 0;
        let mut right = len - 1;

        let mut pivot_idx;
        loop {
            pivot_idx = Self::partition(nums, left, right);
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

    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        let median = Solution::find_kth_largest(nums, n / 2);

        let mut i = 0;
        let mut low = 0;
        let mut high = n - 1;

        while i <= high {
            let tmp = nums[map_idx!(i, n)];
            if tmp < median {
                nums.swap(map_idx!(i, n), map_idx!(high, n));
                high -= 1;
            } else if tmp > median {
                nums.swap(map_idx!(i, n), map_idx!(low, n));
                i += 1;
                low += 1;
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    let mut v = vec![1,4,3,4,1,2,1,3,1,3,2,3,3];
    Solution::wiggle_sort(&mut v);
    println!("{:?}", v);
}
