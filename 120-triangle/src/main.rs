struct Solution;
impl Solution {
    pub fn recursive(triangle: &Vec<Vec<i32>>, tmp: &mut Vec<i32>, i: i32, n: i32) {
        if i == n {
            return;
        }

        tmp[i as usize] = tmp[i as usize - 1] + triangle[i as usize][i as usize];
        for idx in (1..(i as usize)).rev() {
            tmp[idx] = std::cmp::min(
                tmp[idx - 1] + triangle[i as usize][idx],
                tmp[idx] + triangle[i as usize][idx],
            );
        }
        tmp[0] = tmp[0] + triangle[i as usize][0];
        Solution::recursive(triangle, tmp, i + 1, n);
    }

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let len = triangle.len();
        let mut v = vec![0; len];
        if len == 1 {
            return triangle[0][0];
        }
        v[0] = triangle[0][0];
        Solution::recursive(&triangle, &mut v, 1, len as i32);
        *v.iter().min().unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7]])
    );
}
