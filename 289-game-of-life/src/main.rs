struct Solution;
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();

        for i in 0..m {
            for j in 0..n {
                let mut total = 0;

                for ii in (i as i32) - 1..=(i as i32) + 1 {
                    for jj in (j as i32) - 1..=(j as i32) + 1 {
                        if ii == i as i32 && jj == j as i32 {
                            continue;
                        }
                        if ii >= 0
                            && ii < m as i32
                            && jj >= 0
                            && jj < n as i32
                            && board[ii as usize][jj as usize] > 0
                        {
                            total += 1;
                        }
                    }
                }

                if board[i][j] == 0 {
                    if total == 3 {
                        // alive later
                        board[i][j] = -1;
                    }
                } else {
                    if total != 2 && total != 3 {
                        // die later
                        board[i][j] = 2;
                    }
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == -1 {
                    board[i][j] = 1;
                } else if board[i][j] == 2 {
                    board[i][j] = 0;
                }
            }
        }
    }
}

fn main() {
    let mut board = vec![vec![1, 1], vec![1, 0]];
    Solution::game_of_life(&mut board);
    println!("{:?}", board);
}
