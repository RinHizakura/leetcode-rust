struct Solution;
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut ans = vec![0; len];

        let mut stack = Vec::new();
        for idx in (0..len).rev() {
            stack.push(nums[idx]);
        }

        for idx in (0..len).rev() {
            let mut pick = -1;
            while let Some(last) = stack.last() {
                if *last <= nums[idx] {
                    stack.pop();
                } else {
                    pick = *last;
                    break;
                }
            }
            ans[idx] = pick;
            stack.push(nums[idx]);
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::next_greater_elements(vec![5, 4, 3, 2, 1]));
}
