struct Solution;
impl Solution {
    pub fn recursive_solver(
        ans: &mut Vec<Vec<i32>>,
        tmp: &mut Vec<i32>,
        n: i32,
        k: i32,
        i: i32,
        j: i32,
    ) {
        if i == k {
            ans.push(tmp.clone());
            return;
        }

        for idx in j..n {
            tmp.push(idx + 1);
            Solution::recursive_solver(ans, tmp, n, k, i + 1, idx + 1);
            tmp.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut tmp = Vec::new();

        Solution::recursive_solver(&mut ans, &mut tmp, n, k, 0, 0);
        ans
    }
}
fn main() {
    println!("{:?}", Solution::combine(4, 2));
}
