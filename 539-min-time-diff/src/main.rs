struct Solution;
impl Solution {
    fn parse(s: &[u8]) -> i32 {
        ((s[0] - b'0') as i32 * 10 + (s[1] - b'0') as i32) * 60
            + ((s[3] - b'0') as i32 * 10 + (s[4] - b'0') as i32)
    }

    fn time_diff(a: i32, b: i32) -> i32 {
        let diff = b - a;
        diff.min((24 * 60) - diff)
    }

    pub fn find_min_difference(mut time_points: Vec<String>) -> i32 {
        let len = time_points.len();

        time_points.sort();

        let mut idx = 0;
        let mut prev = Self::parse(time_points[len - 1].as_bytes());
        let mut cur = Self::parse(time_points[idx].as_bytes());

        let mut min = Self::time_diff(cur, prev);

        while idx < (len - 1) {
            prev = cur;
            idx += 1;
            cur = Self::parse(time_points[idx].as_bytes());
            min = min.min(Self::time_diff(prev, cur));

            // micro-optimization
            if min == 0 {
                break;
            }
        }

        min
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_min_difference(vec!["00:00".to_string(), "23:59".to_string()])
    );
}
