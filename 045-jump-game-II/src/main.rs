struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dis = vec![i32::MAX; len];

        dis[0] = 0;

        for i in 0..len {
            for step in 1..=nums[i] {
                if (i + step as usize) < len {
                    if dis[i + step as usize] > dis[i] + 1 {
                        dis[i + step as usize] = dis[i] + 1;
                    }
                }
            }
        }

        dis[len - 1]
    }
}

fn main() {
    println!("{}", Solution::jump(vec![2, 3, 1, 1, 4]));
}
