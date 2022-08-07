struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let idx = nums.binary_search(&target);

        if idx.is_err() {
            return vec![-1, -1];
        }

        let mut first = idx.unwrap();
        let mut step = 1;
        while step != 0 {
            if first >= step && nums[first - step] == target {
                first -= step;
                step <<= 1;
            } else {
                step >>= 1;
            }
        }

        let mut last = idx.unwrap();
        let mut step = 1;
        while step != 0 {
            if last + step < len && nums[last + step] == target {
                last += step;
                step <<= 1;
            } else {
                step >>= 1;
            }
        }

        vec![first as i32, last as i32]
    }
}

fn main() {
    println!("{:?}", Solution::search_range(vec![2, 2], 2));
}
