struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let total: i32 = citations.len() as i32;
        let mut start: i32 = 0;
        let mut end: i32 = total as i32 - 1;

        let mut h = 0;
        while start <= end {
            let idx = (start + end) / 2;
            if citations[idx as usize] >= (total - idx) as i32 {
                h = total - idx;
                end = idx - 1;
            } else {
                start = idx + 1;
            }
        }

        h
    }
}

fn main() {
    println!("{}", Solution::h_index(vec![2, 3, 4, 4]));
}
