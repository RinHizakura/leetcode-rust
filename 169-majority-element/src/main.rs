struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut major = 0;
        let mut cnt = 0;

        for n in nums {
            if cnt == 0 {
                major = n;
                cnt = 1;
            } else if n == major {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        major
    }
}

fn main() {
    println!("{}", Solution::majority_element(vec![3, 3, 2]));
}
