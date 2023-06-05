struct Solution;
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut idx = 0;
        let mut ans = Vec::new();

        while idx < len {
            if (nums[idx] != -1) && ((nums[idx] as usize - 1) != idx) {
                if nums[nums[idx] as usize - 1] == nums[idx] {
                    ans.push(nums[idx]);
                    nums[idx] = -1;
                    idx += 1;
                } else {
                    let j = nums[idx] as usize - 1;
                    nums.swap(j, idx);
                }
            } else {
                idx += 1;
            }
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::find_duplicates(vec![1, 2, 2]));
}
