struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let l = nums.len();
        let mut max = nums[0];
        for i in 1..l {
            if max < i as i32 {
                break;
            }

            max = std::cmp::max(max, i as i32 + nums[i]);
        }

        max >= (l - 1) as i32
    }
}

fn main() {
    println!("{}", Solution::can_jump(vec![1, 0]));
    println!("{}", Solution::can_jump(vec![3, 2, 1, 0, 4]));
}
