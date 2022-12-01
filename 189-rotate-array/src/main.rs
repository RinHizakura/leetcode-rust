struct Solution;
impl Solution {
    pub fn rev(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        if k == 0 {
            return;
        }

        nums.reverse();
        Self::rev(nums, 0, k - 1);
        Self::rev(nums, k, len - 1);
    }
}

fn main() {
    let mut v = vec![1,2,3,4,5,6,7];
    Solution::rotate(&mut v, 3);
    println!("{:?}", v);
}
