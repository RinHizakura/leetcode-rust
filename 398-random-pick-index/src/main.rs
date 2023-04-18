use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct Solution {
    map: HashMap<i32, Vec<i32>>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for (idx, n) in nums.into_iter().enumerate() {
            if let Some(v) = map.get_mut(&n) {
                v.push(idx as i32);
            } else {
                let v = vec![idx as i32];
                map.insert(n, v);
            }
        }

        Solution { map }
    }

    fn pick(&self, target: i32) -> i32 {
        if let Some(v) = self.map.get(&target) {
            let len = v.len();
            let mut rng = thread_rng();
            let x: usize = rng.gen::<usize>() % len;
            return v[x];
        }

        unreachable!()
    }
}

fn main() {
    let obj = Solution::new(vec![1, 2, 3, 3, 3]);
    let ret_1: i32 = obj.pick(3);
    println!("{}", ret_1);
}
