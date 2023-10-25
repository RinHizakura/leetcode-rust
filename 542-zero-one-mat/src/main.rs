struct Solution;
impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();

        // scan from top left
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] != 0 {
                    mat[i][j] = i32::MAX;

                    // update according to its left
                    if j > 0 {
                        if mat[i][j - 1] != i32::MAX {
                            mat[i][j] = (mat[i][j - 1] + 1).min(mat[i][j]);
                        }
                    }

                    // update according to its top
                    if i > 0 {
                        if mat[i - 1][j] != i32::MAX {
                            mat[i][j] = (mat[i - 1][j] + 1).min(mat[i][j]);
                        }
                    }
                }
            }
        }

        // scan from bottom right
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if mat[i][j] != 0 {
                    // update according to its right
                    if j + 1 < n {
                        if mat[i][j + 1] != i32::MAX {
                            mat[i][j] = (mat[i][j + 1] + 1).min(mat[i][j]);
                        }
                    }

                    // update according to its bottom
                    if i + 1 < m {
                        if mat[i + 1][j] != i32::MAX {
                            mat[i][j] = (mat[i + 1][j] + 1).min(mat[i][j]);
                        }
                    }
                }
            }
        }

        mat
    }
}

fn main() {
    println!("{:?}", Solution::update_matrix(vec![vec![0, 1, 0]]));
}
