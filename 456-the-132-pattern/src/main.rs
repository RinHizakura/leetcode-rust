struct Solution;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut min = i32::MAX;
        let mut stack: Vec<(i32, i32)> = Vec::new();

        for n in nums {
            let mut top = stack.len();
            // Find the near index j that nums[j] > nums[k]
            while top > 0 && stack[top - 1].0 <= n {
                stack.pop();
                top -= 1;
            }

            if top > 0 {
                // Check if there's nums[i] satisfy nums[k] > nums[i]
                if stack[top - 1].1 < n {
                    return true;
                }
            }
            min = min.min(n);
            stack.push((n, min));
        }

        return false;
    }
}

fn main() {
    println!("{}", Solution::find132pattern(vec![1, 3, 2]));
}
