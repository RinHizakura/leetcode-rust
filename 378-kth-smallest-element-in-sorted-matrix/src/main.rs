struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = matrix.len();
        let mut low = matrix[0][0];
        let mut high = matrix[n - 1][n - 1];

        while low <= high {
            let mid = (high + low) / 2;

            /* Count how many numbers are smaller than /equal to
             * the mid of the range. */
            let mut count = 0;
            let mut j: i32 = (n - 1) as i32;
            for i in 0..n {
                /* If matrix[i][j] > mid, then matrix[i + 1][j] > mid is always
                 * correct. We can make use of this property */
                while j >= 0 && matrix[i][j as usize] > mid {
                    j -= 1;
                }
                count += j as usize + 1;
            }

            /* Shrink sthe range to get close to the answer */
            if count >= k {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        /* When low > high, the last number of mid is the answer we want */
        low
    }
}

fn main() {
    let v = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
    println!("{}", Solution::kth_smallest(v, 8));
}
