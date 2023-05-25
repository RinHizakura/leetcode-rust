struct Solution;
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let len = intervals.len();

        intervals.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut end = intervals[0][1];
        let mut remove = 0;

        for i in 1..len {
            if intervals[i][0] >= end {
                end = intervals[i][1];
            } else {
                remove += 1;
            }
        }

        remove
    }
}

fn main() {
    println!(
        "{}",
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]])
    );
}
