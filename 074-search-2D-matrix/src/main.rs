struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let col: Vec<i32> = matrix.iter().map(|x| x[n - 1]).collect();
        let index = col.binary_search(&target);

        if index.is_ok() {
            return true;
        }

        let pos = index.unwrap_err();
        if pos >= m {
            return false;
        }

        return matrix[pos as usize].binary_search(&target).is_ok();
    }
}

fn main() {
    let matrix = vec![vec![1]];
    println!("{}", Solution::search_matrix(matrix, 0));
}
