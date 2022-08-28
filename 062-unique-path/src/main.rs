struct Solution;
impl Solution {
    fn count_combinations(n: i64, r: i64) -> i64 {
        let r = r.min(n - r);
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let t = (m - 1) + (n - 1);
        Solution::count_combinations(t as i64, (m - 1) as i64) as i32
    }
}

fn main() {
    println!("{}", Solution::unique_paths(59, 5));
}
