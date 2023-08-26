use std::collections::HashSet;

struct Solution;
impl Solution {
    fn go(
        nums: &Vec<i32>,
        idx: usize,
        len: usize,
        tmp: &mut Vec<i32>,
        tmp_len: usize,
        ans: &mut HashSet<Vec<i32>>,
    ) {
        if idx == len {
            if tmp_len >= 2 {
                ans.insert(tmp.clone());
            }
            return;
        }

        // pick
        if tmp_len == 0 || tmp[tmp_len - 1] <= nums[idx] {
            tmp.push(nums[idx]);
            Self::go(nums, idx + 1, len, tmp, tmp_len + 1, ans);
            tmp.pop();
        }

        // dont pick
        Self::go(nums, idx + 1, len, tmp, tmp_len, ans);
    }

    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut tmp = vec![];
        let mut ans = HashSet::new();

        Self::go(&nums, 0, nums.len(), &mut tmp, 0, &mut ans);

        return ans.into_iter().collect();
    }
}

fn main() {
    println!("{}", Solution::find_subsequences(vec![4, 5, 6, 6]));
}
