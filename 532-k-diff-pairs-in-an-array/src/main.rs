use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut pick = HashSet::new();
        let mut candidate = HashSet::new();
        let mut ans = 0;

        /* Simply take this as special case */
        if k == 0 {
            for n in nums {
                if !candidate.insert(n) && pick.insert(n) {
                    ans += 1;
                }
            }
            return ans;
        }

        for n in &nums {
            let sum = *n + k;
            candidate.insert(sum);
        }

        for n in &nums {
            if candidate.contains(n) && pick.insert(*n) {
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::find_pairs(vec![1, 2, 3], 1));
}
