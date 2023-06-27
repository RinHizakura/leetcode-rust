struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap();
        nums.iter().map(|&x| x - min).sum()
    }
}

fn main() {
    println!("{}", Solution::min_moves(vec![1, 2, 3]));
}
