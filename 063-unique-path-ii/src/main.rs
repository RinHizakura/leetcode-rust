struct Solution;
impl Solution {
    fn recursive_solver(
        grid: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
        path: &mut Vec<Vec<i32>>,
    ) {
        if i >= m || j >= n {
            return;
        }

        if grid[i][j] == 0 {
            path[i][j] += path[i - 1][j] + path[i][j - 1];
        }

        for ii in i + 1..m {
            if grid[ii][j] == 0 {
                path[ii][j] = path[ii - 1][j] + path[ii][j - 1];
            }
        }

        for jj in j + 1..n {
            if grid[i][jj] == 0 {
                path[i][jj] = path[i - 1][jj] + path[i][jj - 1];
            }
        }

        Solution::recursive_solver(grid, i + 1, j + 1, m, n, path);
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid[0].len();
        let m = obstacle_grid.len();
        let mut path = vec![vec![0; n]; m];

        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        // initialize the path array
        path[0][0] = 1;
        for i in 1..m {
            if obstacle_grid[i][0] == 1 {
                break;
            }
            path[i][0] = 1;
        }
        for j in 1..n {
            if obstacle_grid[0][j] == 1 {
                break;
            }
            path[0][j] = 1;
        }
        Solution::recursive_solver(&obstacle_grid, 1, 1, m, n, &mut path);

        path[m - 1][n - 1]
    }
}

fn main() {
    let v = vec![
        vec![0, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
    ];

    for i in 0..5 {
        println!("{:?}", v[i]);
    }
    println!("");

    println!("{}", Solution::unique_paths_with_obstacles(v));
}
