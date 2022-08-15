struct Solution;
impl Solution {
    pub fn recursive(
        nums: &mut Vec<i32>,
        i: usize,
        len: usize,
        solution: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if i == len {
            ans.push(solution.clone());
            return;
        }

        let mut bit = 0;

        for j in i..len {
            let mark = 1 << (nums[j] + 10);
            if bit & mark != 0 {
                continue;
            }
            bit |= mark;
            solution[i] = nums[j];
            nums.swap(i, j);
            Solution::recursive(nums, i + 1, len, solution, ans);
            nums.swap(i, j);
        }
    }

    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut ans = Vec::new();
        let mut solution = vec![0; len];

        Solution::recursive(&mut nums, 0, len, &mut solution, &mut ans);

        ans
    }
}


fn main() {
    println!("{:?}", Solution::permute_unique(vec![1,2,2]));
}
