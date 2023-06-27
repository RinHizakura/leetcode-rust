struct Solution;
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();

        points.sort_by_key(|k| k[0]);

        let mut ans = 1;
        let mut window_end = points[0][1];
        for i in 1..len {
            let p = &points[i];
            if p[0] > window_end {
                ans += 1;
                window_end = p[1];
            } else {
                window_end = p[1].min(window_end);
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_min_arrow_shots(vec![vec![1, 3], vec![2, 3]])
    );
}
