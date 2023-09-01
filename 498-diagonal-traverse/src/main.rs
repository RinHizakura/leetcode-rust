struct Solution;
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();

        let l = m.max(n) * 2 + 1;

        let mut ans = vec![0; m * n];
        let mut idx = 0;
        let mut start = 0;
        while idx < l {
            for cur in 0..=idx {
                let i = if idx & 1 == 1 { cur } else { idx - cur };
                let j = idx - i;
                if i < m && j < n {
                    ans[start] = mat[i][j];
                    start += 1;
                }
            }

            idx += 1;
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]])
    );
}
