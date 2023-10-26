use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let m = wall.len() as i32;
        let mut v_set: HashMap<i32, i32> = HashMap::new();

        for row in wall {
            let mut next = 0;

            let len = row.len();
            for idx in 0..(len - 1) {
                let brick = row[idx];
                next += brick;
                *v_set.entry(next).or_default() += 1;
            }
        }

        m - *v_set.values().max().unwrap_or(&0)
    }
}

fn main() {
    println!(
        "{}",
        Solution::least_bricks(vec![vec![2147483647], vec![2147483647]])
    );
}
