struct Solution;
impl Solution {
    pub fn recursive(board: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        if board[i][j] == 'X' {
            return;
        }

        board[i][j] = '#';
        // up
        if i > 0 {
            Self::recursive(board, i - 1, j, m, n);
        }
        // down
        if i + 1 < m {
            Self::recursive(board, i + 1, j, m, n);
        }
        // left
        if j > 0 {
            Self::recursive(board, i, j - 1, m, n);
        }
        // down
        if j + 1 < n {
            Self::recursive(board, i, j + 1, m, n);
        }
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();

        for j in 0..n {
            Self::recursive(board, 0, j, m, n);
        }

        for i in 1..m - 1 {
            Self::recursive(board, i, 0, m, n);
            Self::recursive(board, i, n - 1, m, n);
        }

        for j in 0..n {
            Self::recursive(board, m - 1, j, m, n);
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == '#' {
                    board[i][j] = 'O';
                } else {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
fn main() {
    let mut v = vec![vec!['O', 'O'], vec!['O', 'O']];

    Solution::solve(&mut v);
    println!("\nans");
    for i in v {
        println!("{:?}", i);
    }
}
