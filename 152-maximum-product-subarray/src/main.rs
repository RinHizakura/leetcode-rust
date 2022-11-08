struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut min_prod = nums[0];
        let mut max_prod = nums[0];
        let mut ans = nums[0];
        for i in 1..len {
            let max_prod_prev = max_prod;
            max_prod = nums[i].max(nums[i] * max_prod_prev).max(nums[i] * min_prod);
            min_prod = nums[i].min(nums[i] * min_prod).min(nums[i] * max_prod_prev);
            ans = ans.max(max_prod);
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![2, 3, 4, -1, -2]));
}
