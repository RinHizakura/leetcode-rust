struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        let mut next = 1;
        let mut cur = nums[0];
        let mut count = 1;

        for i in 1..len {
            if nums[i] == cur {
                count += 1;
                if count > 2 {
                    continue;
                }
            } else {
                count = 1;
            }

            nums[next] = nums[i];
            cur = nums[i];
            next += 1;
        }

        next as i32
    }
}

fn main() {
    let mut v = vec![1, 1, 1, 2, 2, 3];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", v);
}
