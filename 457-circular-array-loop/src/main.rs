struct Solution;
impl Solution {
    fn is_cycle<F>(
        v: usize,
        n: usize,
        nums: &Vec<i32>,
        visited: &mut Vec<bool>,
        on_path: &mut Vec<bool>,
        dir: F,
    ) -> bool
    where
        F: Fn(i32) -> bool,
    {
        if visited[v] == false && dir(nums[v]) {
            visited[v] = true;
            on_path[v] = true;

            let mut next_idx = v as i32 + nums[v];
            while next_idx < 0 {
                next_idx += n as i32;
            }
            let next_idx = (next_idx % n as i32) as usize;

            // k = 1 is not taken as cycle here
            if next_idx != v {
                if !visited[next_idx] && Self::is_cycle(next_idx, n, nums, visited, on_path, dir) {
                    return true;
                }

                if on_path[next_idx] {
                    return true;
                }
            }
        }

        on_path[v] = false;
        return false;
    }

    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut visited_pos = vec![false; n];
        let mut visited_neg = vec![false; n];
        let mut on_path = vec![false; n];

        for v in 0..n {
            if visited_pos[v] == false {
                if Self::is_cycle(v, n, &nums, &mut visited_pos, &mut on_path, |n| n > 0) {
                    return true;
                }
            }

            if visited_neg[v] == false {
                if Self::is_cycle(v, n, &nums, &mut visited_neg, &mut on_path, |n| n < 0) {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    println!("{}", Solution::circular_array_loop(vec![1, -1]));
}
