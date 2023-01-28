struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        /* let's start at the top right corner */
        let mut i: i32 = 0;
        let mut j: i32 = (n - 1) as i32;

        while (i as usize) < m && j >= 0 {
            let check = matrix[i as usize][j as usize];
            if check == target {
                return true;
            } else if check > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
        return false;
    }
}

fn main() {
    let m = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];

    println!("{}", Solution::search_matrix(m, 5));
}
