struct Solution;
impl Solution {
    fn dir0(dir: &mut usize, i: &mut usize, j: &mut usize, m1: &mut usize, n0: &mut usize) {
        if *j + 1 >= *m1 {
            *i += 1;
            *dir = 1;
            *n0 += 1;
        } else {
            *j += 1;
        }
    }

    fn dir1(dir: &mut usize, i: &mut usize, j: &mut usize, n1: &mut usize, m1: &mut usize) {
        if *i + 1 >= *n1 {
            *j -= 1;
            *dir = 2;
            *m1 -= 1;
        } else {
            *i += 1;
        }
    }

    fn dir2(dir: &mut usize, i: &mut usize, j: &mut usize, m0: &mut usize, n1: &mut usize) {
        if *j == *m0 {
            *i -= 1;
            *dir = 3;
            *n1 -= 1;
        } else {
            *j -= 1;
        }
    }

    fn dir3(dir: &mut usize, i: &mut usize, j: &mut usize, n0: &mut usize, m0: &mut usize) {
        if *i == *n0 {
            *j += 1;
            *dir = 0;
            *m0 += 1;
        } else {
            *i -= 1;
        }
    }

    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut m0 = 0;
        let mut n0 = 0;
        let mut m1 = matrix[0].len();
        let mut n1 = matrix.len();
        let mut cnt = m1 * n1;

        let mut i = 0;
        let mut j = 0;

        let mut dir = 0;

        loop {
            v.push(matrix[i][j]);
            cnt -= 1;

            match dir {
                0 => Solution::dir0(&mut dir, &mut i, &mut j, &mut m1, &mut n0),
                1 => Solution::dir1(&mut dir, &mut i, &mut j, &mut n1, &mut m1),
                2 => Solution::dir2(&mut dir, &mut i, &mut j, &mut m0, &mut n1),
                3 => Solution::dir3(&mut dir, &mut i, &mut j, &mut n0, &mut m0),
                _ => panic!(),
            }

            if cnt == 0 {
                break;
            }
        }

        v
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}
