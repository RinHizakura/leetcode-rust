struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        // 200 / 64 = 4

        let mut row_bit: [u64; 4] = [0; 4];
        let mut col_bit: [u64; 4] = [0; 4];

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    row_bit[i / 64] |= 1 << (i % 64);
                    col_bit[j / 64] |= 1 << (j % 64);
                }
            }
        }

        for i in 0..m {
            let r = (row_bit[i / 64] >> (i % 64)) & 1;
            for j in 0..n {
                let c = (col_bit[j / 64] >> (j % 64)) & 1;
                if r == 1 || c == 1 {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

fn main() {
    let mut m = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut m);
    println!("{:?}", m);
}
