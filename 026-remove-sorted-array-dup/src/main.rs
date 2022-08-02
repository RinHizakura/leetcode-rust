struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        let mut next = 1;
        let mut cur = nums[0];
        for i in 1..len {
            if nums[i] == cur {
                continue;
            }

            nums[next] = nums[i];
            cur = nums[i];
            next += 1;
        }

        next as i32
    }
}

fn main() {
    println!("{}", Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));
}
