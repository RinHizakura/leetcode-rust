struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();

        let mut left = 0;
        let mut right = 0;
        let mut sum = 0;

        let mut ans = i32::MAX;

        loop {
            if sum >= target {
                ans = ans.min((right - left) as i32);
                sum -= nums[left];
                left += 1;
            } else {
                if right >= len {
                    break;
                }
                sum += nums[right];
                right += 1;
            }
        }

        if ans != i32::MAX {
            ans
        } else {
            0
        }
    }
}

fn main() {
    println!("{}", Solution::min_sub_array_len(7, vec![2,1,3,2,3,4]));
}
