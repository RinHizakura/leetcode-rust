struct Solution;
impl Solution {
    fn recursive_solver(
        board: &Vec<Vec<char>>,
        map: u64,
        m: usize,
        n: usize,
        i: usize,
        j: usize,
        word: &Vec<char>,
        str_i: usize,
        str_len: usize,
    ) -> bool {
        let mut found = false;
        if (board[i][j] == word[str_i]) && ((map & (1 << (i + j * m))) == 0) {
            if str_len == str_i + 1 {
                return true;
            }

            let map = map | (1 << (i + j * m));
            if i + 1 < m {
                found = Solution::recursive_solver(
                    &board,
                    map,
                    m,
                    n,
                    i + 1,
                    j,
                    &word,
                    str_i + 1,
                    str_len,
                );
            }
            if found == true {
                return true;
            }

            if i > 0 {
                found = Solution::recursive_solver(
                    &board,
                    map,
                    m,
                    n,
                    i - 1,
                    j,
                    &word,
                    str_i + 1,
                    str_len,
                );
            }
            if found == true {
                return true;
            }

            if j + 1 < n {
                found = Solution::recursive_solver(
                    &board,
                    map,
                    m,
                    n,
                    i,
                    j + 1,
                    &word,
                    str_i + 1,
                    str_len,
                );
            }
            if found == true {
                return true;
            }

            if j > 0 {
                found = Solution::recursive_solver(
                    &board,
                    map,
                    m,
                    n,
                    i,
                    j - 1,
                    &word,
                    str_i + 1,
                    str_len,
                );
            }
            if found == true {
                return true;
            }
        }

        return false;
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        // 1 <= m, n <= 6
        let m = board.len();
        let n = board[0].len();
        let str_len = word.len();
        let word = word.chars().collect::<Vec<_>>();

        let mut found = false;
        for i in 0..m {
            for j in 0..n {
                found = Solution::recursive_solver(&board, 0, m, n, i, j, &word, 0, str_len);
                if found == true {
                    return true;
                }
            }
        }

        found
    }
}

fn main() {
    let v = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'E', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    println!("{}", Solution::exist(v, "ABCESEEEFS".to_string()));
}
