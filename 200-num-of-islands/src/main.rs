struct Solution;
impl Solution {
    fn reach(grid: &mut Vec<Vec<char>>, i: i32, j: i32, m: i32, n: i32) {
        if i >= 0 && i < m && j >= 0 && j < n && grid[i as usize][j as usize] == '1' {
            grid[i as usize][j as usize] = '2';
            Self::reach(grid, i + 1, j, m, n);
            Self::reach(grid, i - 1, j, m, n);
            Self::reach(grid, i, j + 1, m, n);
            Self::reach(grid, i, j - 1, m, n);
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut cnt = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i as usize][j as usize] == '1' {
                    cnt += 1;
                    Self::reach(&mut grid, i, j, m, n);
                }
            }
        }

        cnt
    }
}

fn main() {
    let v = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    println!("{}", Solution::num_islands(v));
}
