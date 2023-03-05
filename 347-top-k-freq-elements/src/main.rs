use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        /* Use a hashmap to record each number's occurrence */
        let mut map: HashMap<i32, usize> = HashMap::new();
        for n in nums.iter() {
            if let Some(x) = map.get_mut(n) {
                *x = *x + 1;
            } else {
                map.insert(*n, 1);
            }
        }

        let mut v: Vec<(i32, usize)> = map.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        let v: Vec<i32> = v.iter().map(|(n, _)| *n).collect();

        v[0..k].to_vec()
    }
}

fn main() {
    println!("{:?}", Solution::top_k_frequent(vec![3, 0, 1, 0], 1));
}
