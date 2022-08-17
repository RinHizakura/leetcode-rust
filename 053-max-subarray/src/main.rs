struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        let mut p = 0;
        let mut ans = nums[0];
        let mut sum = nums[0];

        for i in 1..len {
            /* if this is a positive number, making the sum as large
             * as possible */
            if nums[i] > 0 {
                while nums[p] < 0 && p < i {
                    sum -= nums[p];
                    p += 1;
                }
            }

            /* if this number cause the sum being negative, making the sum
             * being positive as possible as we can */
            if sum + nums[i] < 0 {
                while sum + nums[i] < 0 && p < i {
                    sum -= nums[p];
                    p += 1;
                }
            }

            sum += nums[i];

            if sum > ans {
                ans = sum;
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!("{}", Solution::max_sub_array(vec![5, 4, -1, 7, 8]));
}
