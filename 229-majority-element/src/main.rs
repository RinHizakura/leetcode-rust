struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut x = -1;
        let mut x_cnt = 0;

        let mut y = -1;
        let mut y_cnt = 0;

        for num in nums.iter() {
            // If this num is same as x, push num into stack x
            if x_cnt > 0 && *num == x {
                x_cnt += 1;
            }
            // If this num is same as y, push num into stack y
            else if y_cnt > 0 && *num == y {
                y_cnt += 1;
            }
            // If stack x is empty, push num into stack x
            else if x_cnt == 0 {
                x = *num;
                x_cnt += 1;
            }
            // If stack y is empty, push num into stack y
            else if y_cnt == 0 {
                y = *num;
                y_cnt += 1;
            }
            // otherwise, this num is different with both x and y, form a triplet with them
            else {
                x_cnt -= 1;
                y_cnt -= 1;
            }
        }

        let mut x_cnt = 0;
        let mut y_cnt = 0;
        for num in nums.iter() {
            if *num == x {
                x_cnt += 1;
            } else if *num == y {
                y_cnt += 1;
            }
        }

        let major_cnt = nums.len() / 3;
        let mut v = Vec::new();
        if x_cnt > major_cnt {
            v.push(x);
        }
        if y_cnt > major_cnt {
            v.push(y);
        }
        v
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::majority_element(vec![2, 1, 1, 3, 1, 4, 5, 6])
    );
}
