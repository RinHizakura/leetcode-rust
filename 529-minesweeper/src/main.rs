struct Solution;
impl Solution {
    fn check(board: &mut Vec<Vec<char>>, i: i32, j: i32, m: i32, n: i32) -> u8 {
        if i >= m || j >= n || i < 0 || j < 0 {
            return 0;
        }

        let i = i as usize;
        let j = j as usize;

        match board[i][j] {
            'M' => return 1,
            _ => {}
        }

        return 0;
    }

    fn walk(board: &mut Vec<Vec<char>>, i: i32, j: i32, m: i32, n: i32) {
        if i >= m || j >= n || i < 0 || j < 0 {
            return;
        }

        if board[i as usize][j as usize] != 'E' {
            return;
        }

        let mut sum: u8 = 0;

        sum += Self::check(board, i + 1, j, m, n);
        sum += Self::check(board, i - 1, j, m, n);
        sum += Self::check(board, i, j + 1, m, n);
        sum += Self::check(board, i, j - 1, m, n);
        sum += Self::check(board, i + 1, j - 1, m, n);
        sum += Self::check(board, i + 1, j + 1, m, n);
        sum += Self::check(board, i - 1, j - 1, m, n);
        sum += Self::check(board, i - 1, j + 1, m, n);

        if sum == 0 {
            board[i as usize][j as usize] = 'B';
            Self::walk(board, i + 1, j, m, n);
            Self::walk(board, i - 1, j, m, n);
            Self::walk(board, i, j + 1, m, n);
            Self::walk(board, i, j - 1, m, n);
            Self::walk(board, i + 1, j - 1, m, n);
            Self::walk(board, i + 1, j + 1, m, n);
            Self::walk(board, i - 1, j - 1, m, n);
            Self::walk(board, i - 1, j + 1, m, n);
        } else {
            board[i as usize][j as usize] = (sum + b'0') as char;
        }
    }

    fn click(board: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        if board[i][j] == 'M' {
            board[i][j] = 'X';
            return;
        }

        // should only click 'E'
        Self::walk(board, i as i32, j as i32, m as i32, n as i32);
    }

    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let m = board.len();
        let n = board[0].len();

        Self::click(&mut board, click[0] as usize, click[1] as usize, m, n);

        board
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::update_board(
            vec![
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'M', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E']
            ],
            vec![3, 0]
        )
    );
}
