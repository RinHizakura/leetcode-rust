struct Solution;
impl Solution {
    fn recursive_solver(
        grid: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
        accum: &mut Vec<Vec<i32>>,
    ) {
        if i >= m || j >= n {
            return;
        }

        accum[i][j] = accum[i][j].min(accum[i - 1][j] + grid[i][j]);
        accum[i][j] = accum[i][j].min(accum[i][j - 1] + grid[i][j]);
        for ii in i + 1..m {
            accum[ii][j] = accum[ii][j].min(accum[ii - 1][j] + grid[ii][j]);
            accum[ii][j] = accum[ii][j].min(accum[ii][j - 1] + grid[ii][j]);
        }
        for jj in j + 1..n {
            accum[i][jj] = accum[i][jj].min(accum[i][jj - 1] + grid[i][jj]);
            accum[i][jj] = accum[i][jj].min(accum[i - 1][jj] + grid[i][jj]);
        }
        Solution::recursive_solver(grid, i + 1, j + 1, m, n, accum);
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut accum = vec![vec![i32::MAX; n]; m];

        accum[0][0] = grid[0][0];
        for i in 1..n {
            accum[0][i] = accum[0][i].min(accum[0][i - 1] + grid[0][i]);
        }
        for j in 1..m {
            accum[j][0] = accum[j][0].min(accum[j - 1][0] + grid[j][0]);
        }
        Solution::recursive_solver(&grid, 1, 1, m, n, &mut accum);

        accum[m - 1][n - 1]
    }
}

fn main() {
    let v = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];

    println!("{}", Solution::min_path_sum(v));
}
