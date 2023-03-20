struct Solution;
impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut sum = vec![0; target + 1];

        nums.sort();

        for i in 0..=target {
            for n in &nums {
                let n_u = *n as usize;
                if n_u > i {
                    break;
                } else if n_u == i {
                    sum[i] += 1;
                    break;
                }
                sum[i] += sum[i - n_u];
            }
        }

        sum[target] as i32
    }
}

fn main() {
    println!("{}", Solution::combination_sum4(vec![1, 2, 3], 4));
}
