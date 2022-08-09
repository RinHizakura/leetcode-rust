struct Solution;
impl Solution {
    fn recursive_solver(
        candidates: &Vec<i32>,
        len: usize,
        target: i32,
        i: usize,
        mut sum: i32,
        ans: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if i == 0 {
            return;
        }

        let cnt = (target - sum) / candidates[i - 1];

        for _ in 0..cnt as usize {
            ans.push(candidates[i - 1]);
            sum += candidates[i - 1];
        }

        for _ in 0..cnt as usize {
            if sum == target {
                ret.push(ans.clone());
            } else {
                Solution::recursive_solver(candidates, len, target, i - 1, sum, ans, ret);
            }
            sum -= candidates[i - 1];
            ans.pop();
        }

        Solution::recursive_solver(candidates, len, target, i - 1, sum, ans, ret);
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        let len = candidates.len();
        let mut ans = Vec::new();
        let mut ret = Vec::new();

        Solution::recursive_solver(&candidates, len, target, len, 0, &mut ans, &mut ret);
        ret
    }
}

fn main() {
    println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
}
