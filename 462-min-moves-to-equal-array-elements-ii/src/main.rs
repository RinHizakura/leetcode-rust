struct Solution;
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mid = nums.len() / 2;
        let median = nums[mid];

        nums.iter().fold(0, |acc, x| acc + (x - median).abs())
    }
}

fn main() {
    println!("{}", Solution::min_moves2(vec![0, 1, 1, 2]));
}
