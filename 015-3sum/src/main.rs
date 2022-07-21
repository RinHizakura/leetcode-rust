struct Solution;
impl Solution {
    fn inc(nums: &Vec<i32>, i: usize, len: usize) -> usize {
        let n = nums[i];
        let mut i = i + 1;
        while i < len && n == nums[i] {
            i += 1;
        }

        i
    }

    fn dec(nums: &Vec<i32>, i: usize) -> usize {
        let n = nums[i];

        let mut i = i - 1;
        while i > 0 && n == nums[i] {
            i -= 1;
        }

        i
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut nums = nums.clone();
        nums.sort();

        let mut ans_vec = Vec::new();
        for i in 0..len {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    ans_vec.push(vec![nums[i], nums[left], nums[right]]);
                    left = Solution::inc(&nums, left, len);
                    right = Solution::dec(&nums, right);
                } else if sum < 0 {
                    left = Solution::inc(&nums, left, len);
                } else {
                    right = Solution::dec(&nums, right);
                }
            }
        }

        ans_vec
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
    println!("{:?}", Solution::three_sum(vec![0, 0, 0]));
}
