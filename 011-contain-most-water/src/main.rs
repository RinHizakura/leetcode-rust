use std::cmp;

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut right = height.len() - 1;
        let mut left = 0;

        let mut max = cmp::min(height[left], height[right]) as usize * (right - left);
        while left < right {
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }

            let tmp = cmp::min(height[left], height[right]) as usize * (right - left);
            if max < tmp {
                max = tmp;
            }
        }

        return max as i32;
    }
}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
