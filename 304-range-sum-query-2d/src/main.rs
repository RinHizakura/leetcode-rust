struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}
impl NumMatrix {
    fn eval(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        matrix[i][j] + matrix[i - 1][j] + matrix[i][j - 1] - matrix[i - 1][j - 1]
    }

    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 1..m {
            matrix[i][0] = matrix[i - 1][0] + matrix[i][0];
        }

        for i in 1..n {
            matrix[0][i] = matrix[0][i - 1] + matrix[0][i];
        }

        for i in 1..m.min(n) {
            matrix[i][i] = Self::eval(&matrix, i, i);
            for j in (i + 1)..n {
                matrix[i][j] = Self::eval(&matrix, i, j);
            }

            for j in (i + 1)..m {
                matrix[j][i] = Self::eval(&matrix, j, i);
            }
        }

        NumMatrix { matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        let mut exclude_top = 0;
        if row1 > 0 {
            exclude_top = self.matrix[row1 - 1][col2];
        }
        let mut exclude_left = 0;
        if col1 > 0 {
            exclude_left = self.matrix[row2][col1 - 1];
        }

        let mut overlap = 0;
        if row1 > 0 && col1 > 0 {
            overlap = self.matrix[row1 - 1][col1 - 1];
        }

        self.matrix[row2][col2] - exclude_top - exclude_left + overlap
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![3, 3, 3]];
    let obj = NumMatrix::new(matrix);
    let ret_1: i32 = obj.sum_region(1, 1, 1, 1);
    println!("{}", ret_1);
}
