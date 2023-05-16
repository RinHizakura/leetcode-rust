struct Solution;
impl Solution {
    fn walk(board: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) -> Result<(), ()> {
        if board[i][j] == '.' {
            return Ok(());
        }

        board[i][j] = '.';

        if i > 0 {
            Self::walk(board, i - 1, j, m, n)?;
        }
        if i < m - 1 {
            Self::walk(board, i + 1, j, m, n)?;
        }
        if j > 0 {
            Self::walk(board, i, j - 1, m, n)?;
        }
        if j < n - 1 {
            Self::walk(board, i, j + 1, m, n)?;
        }

        Err(())
    }

    pub fn count_battleships(mut board: Vec<Vec<char>>) -> i32 {
        let m = board.len();
        let n = board[0].len();

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if board[i][j] != 'X' {
                    continue;
                }

                Self::walk(&mut board, i, j, m, n);
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::count_battleships(vec![vec!['X', 'X', 'X']]));
}
