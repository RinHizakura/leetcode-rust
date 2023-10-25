struct Solution;
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();

        let mut total = n;
        let mut group_id = vec![0; n];

        for i in 1..n {
            group_id[i] = i;

            for j in 0..i {
                if is_connected[i][j] == 1 {
                    let group_i = group_id[i];
                    let group_j = group_id[j];

                    /* Merge this two differnet group together */
                    if group_i != group_j {
                        total -= 1;
                    }

                    group_id.iter_mut().for_each(|x| {
                        if *x == group_i {
                            *x = group_j
                        }
                    });
                }
            }
        }

        total as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_circle_num(vec![vec![1, 0], vec![0, 1]])
    );
}
