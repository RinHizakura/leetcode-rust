struct Solution;
impl Solution {
    pub fn recursive_solver(
        ans: &mut Vec<Vec<i32>>,
        tmp: &mut Vec<i32>,
        nums: &Vec<i32>,
        i: usize,
        len: usize,
    ) {
        if i >= len {
            return;
        }

        for idx in i..len {
            tmp.push(nums[idx]);
            ans.push(tmp.clone());
            Solution::recursive_solver(ans, tmp, nums, idx + 1, len);
            tmp.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut tmp = vec![];

        let len = nums.len();
        ans.push(vec![]);

        Solution::recursive_solver(&mut ans, &mut tmp, &nums, 0, len);
        ans
    }
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", Solution::subsets(v));
}
