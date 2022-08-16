struct Solution;
impl Solution {
    pub fn spin(matrix: &mut Vec<Vec<i32>>, i: usize, n: usize, len: usize) {
        if i >= n {
            return;
        }

        let (x1, mut y1) = (i, i);
        let (mut x2, y2) = (i, n);
        let (x3, mut y3) = (n, n);
        let (mut x4, y4) = (n, i);

        for _ in i..n {
            let a = matrix[x1][y1];
            let b = matrix[x2][y2];
            let c = matrix[x3][y3];
            let d = matrix[x4][y4];
            matrix[x1][y1] = d;
            matrix[x2][y2] = a;
            matrix[x3][y3] = b;
            matrix[x4][y4] = c;

            y1 += 1;
            x2 += 1;
            y3 -= 1;
            x4 -= 1;
        }
        Solution::spin(matrix, i + 1, n - 1, (len + 1) / 2);
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        Solution::spin(matrix, 0, len - 1, len);
    }
}

fn main() {
    let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let n = m.len();
    println!("Before:");
    for i in 0..n {
        println!("{:?}", m[i]);
    }
    Solution::rotate(&mut m);
    println!("After:");
    for i in 0..n {
        println!("{:?}", m[i]);
    }
}
