struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = intervals.len();

        intervals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        let mut ans = Vec::new();

        for v in intervals.into_iter() {
            if ans.is_empty() {
                ans.push(v);
                continue;
            }

            let last = ans.last_mut().unwrap();

            if v[0] > last[1] {
                ans.push(v);
            } else {
                last[1] = std::cmp::max(last[1], v[1]);
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
}
