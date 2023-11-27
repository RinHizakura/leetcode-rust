struct Solution;
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by_key(|x| x[1]);

        let mut ans = 0;
        let mut prev_end = -1001;
        for pair in pairs {
            // greedily pick the possible pair
            if pair[0] > prev_end {
                ans += 1;
                prev_end = pair[1];
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_longest_chain(vec![vec![0, 1], vec![1, 2]])
    );
}
