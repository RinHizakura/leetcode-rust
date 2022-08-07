struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_valid = [0; 9];
        let mut col_valid = [0; 9];
        let mut block_valid = [0; 9];

        for i in 0..9 {
            for j in 0..9 {
                let target = board[i][j];
                if target == '.' {
                    continue;
                }

                let target = target as u8 - '0' as u8;
                if row_valid[i] & 1 << target != 0 {
                    return false;
                } else {
                    row_valid[i] |= 1 << target;
                }

                if col_valid[j] & 1 << target != 0 {
                    return false;
                } else {
                    col_valid[j] |= 1 << target;
                }

                let idx = (i / 3) * 3 + j / 3;
                if block_valid[idx] & 1 << target != 0 {
                    return false;
                } else {
                    block_valid[idx] |= 1 << target;
                }
            }
        }

        true
    }
}

fn main() {
    let v = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    println!("{}", Solution::is_valid_sudoku(v));
}
