struct Solution;
impl Solution {
    pub fn recursive(
        nums: &Vec<i32>,
        i: usize,
        len: usize,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        let mut mask = 0;
        for idx in i..len {
            let val = nums[idx];
            if mask & 1 << val == 0 {
                mask |= 1 << val;
                tmp.push(val);
                ans.push(tmp.clone());
                Solution::recursive(nums, idx + 1, len, tmp, ans);
                tmp.pop();
            }
        }
    }

    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans = vec![vec![]];
        let mut tmp = vec![];
        Solution::recursive(&nums, 0, nums.len(), &mut tmp, &mut ans);
        ans
    }
}

fn main() {
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 2]));
}
