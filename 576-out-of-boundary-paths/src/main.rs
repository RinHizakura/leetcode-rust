struct Solution;
impl Solution {
    fn go(
        i: i32,
        j: i32,
        m: i32,
        n: i32,
        max_move: i32,
        dp: &mut Vec<Vec<Vec<i32>>>,
        modulo: i32,
    ) -> i32 {
        if i < 0 || i >= m || j < 0 || j >= n {
            return 1;
        }

        if max_move == 0 {
            return 0;
        }

        if dp[i as usize][j as usize][(max_move - 1) as usize] != -1 {
            return dp[i as usize][j as usize][(max_move - 1) as usize];
        }

        let mut sol = 0;
        // left
        sol += Self::go(i, j - 1, m, n, max_move - 1, dp, modulo);
        sol %= modulo;

        // right
        sol += Self::go(i, j + 1, m, n, max_move - 1, dp, modulo);
        sol %= modulo;

        // up
        sol += Self::go(i - 1, j, m, n, max_move - 1, dp, modulo);
        sol %= modulo;

        // down
        sol += Self::go(i + 1, j, m, n, max_move - 1, dp, modulo);
        sol %= modulo;

        dp[i as usize][j as usize][(max_move - 1) as usize] = sol;

        return sol;
    }

    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; max_move as usize]; n as usize]; m as usize];
        Self::go(
            start_row,
            start_column,
            m,
            n,
            max_move,
            &mut dp,
            1000_000_000 + 7,
        )
    }
}

fn main() {
    println!("{}", Solution::find_paths(2, 2, 2, 0, 0));
}
