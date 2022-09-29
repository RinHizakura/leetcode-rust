struct Solution;
impl Solution {
    pub fn recursive(left: i32, right: i32, fast_map: &mut Vec<i32>) -> i32 {
        if left >= right {
            return 1;
        }

        if fast_map[(right - left) as usize] != -1 {
            return fast_map[(right - left) as usize];
        }

        let mut sum = 0;
        for i in left..=right {
            let lefts = Solution::recursive(left, i - 1, fast_map);
            let rights = Solution::recursive(i + 1, right, fast_map);
            sum += lefts * rights;
        }
        fast_map[(right - left) as usize] = sum;
        sum
    }

    pub fn num_trees(n: i32) -> i32 {
        let mut fast_map = vec![-1; n as usize];
        Solution::recursive(1, n, &mut fast_map)
    }
}

fn main() {
    println!("{}", Solution::num_trees(3));
}
