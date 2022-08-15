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

        for j in i..len {
            solution[i] = nums[j];
            nums.swap(i, j);
            Solution::recursive(nums, i + 1, len, solution, ans);
            nums.swap(i, j);
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut ans = Vec::new();
        let mut solution = vec![0; len];

        Solution::recursive(&mut nums, 0, len, &mut solution, &mut ans);

        ans
    }
}

fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
}
