struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut nums = nums.clone();
        let mut ans_d = i32::MAX;
        let mut ans = i32::MAX;
        nums.sort();

        for i in 0..len {
            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                let d = sum - target;

                if d == 0 {
                    return sum;
                } else {
                    let abs_d = d.abs();
                    if abs_d < ans_d {
                        ans_d = abs_d;
                        ans = sum;
                    }

                    if d > 0 {
                        right -= 1;
                    } else {
                        left += 1;
                    }
                }
            }
        }

        return ans;
    }
}

fn main() {
    println!("{} 2", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    println!("{} 0", Solution::three_sum_closest(vec![0, 0, 0], 1));
    println!(
        "{} -2",
        Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2)
    );
}
