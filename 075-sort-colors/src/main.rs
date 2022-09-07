struct Solution;
impl Solution {
    pub fn sort_target(nums: &mut Vec<i32>, i: &mut i32, mut switch: i32, target: i32) {
        loop {
            if nums[*i as usize] != target {
                nums.swap(*i as usize, switch as usize);
                switch -= 1;
            } else {
                *i += 1;
            }

            if *i >= switch {
                break;
            }
        }
    }

    pub fn sort_colors(nums: &mut Vec<i32>) {
        let len = nums.len() as i32;
        let mut i = 0;

        Solution::sort_target(nums, &mut i, len - 1, 0);
        if i >= len - 1 {
            return;
        }
        if nums[i as usize] == 0 {
            i += 1;
        }
        Solution::sort_target(nums, &mut i, len - 1, 1);
    }
}

fn main() {
    let mut v = vec![1];
    Solution::sort_colors(&mut v);
    println!("{:?}", v);
}
