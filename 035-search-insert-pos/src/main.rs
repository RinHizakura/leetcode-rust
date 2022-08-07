struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        /* HaHaHa */
        nums.partition_point(|&x| x < target) as i32
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1,3,5,6], 7));
}
