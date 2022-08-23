struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        Solution::merge(intervals)
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        )
    );
}
