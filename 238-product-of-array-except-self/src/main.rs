struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![1; len];

        let mut pre = 1;
        for i in 0..len {
            ans[i] *= pre;
            pre *= nums[i];
        }

        let mut post = 1;
        for i in (0..len).rev() {
            ans[i] *= post;
            post *= nums[i];
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4, 5]));
}
