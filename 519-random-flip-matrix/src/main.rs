use rand::prelude::*;
use std::collections::HashMap;

struct Solution {
    m: i32,
    n: i32,
    total: i32,
    map: HashMap<i32, i32>,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Solution {
            m: m,
            n: n,
            total: m * n,
            map: HashMap::new(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let rand: u32 = rng.gen::<u32>() % self.total as u32;

        self.total -= 1;

        let key;
        if let Some(e) = self.map.get(&(rand as i32)) {
            /* A suggestion for key when choosing this random value is put in the slot */
            key = *e;
        } else {
            /* This random value has not been used as key before. */
            key = rand as i32;
        };

        /* In the next iteration, index self.total lost chance to get pick, let suggest
         * somebody to pick it when they also choose rand */
        if let Some(e) = self.map.get(&self.total) {
            self.map.insert(rand as i32, *e);
        } else {
            self.map.insert(rand as i32, self.total);
        }

        return vec![key / self.n, key % self.n];
    }

    fn reset(&mut self) {
        self.total = (self.m * self.n) as i32;
        self.map.clear();
    }
}

fn main() {
    let mut obj = Solution::new(3, 1);
    let ret_1: Vec<i32> = obj.flip();
    println!("{ret_1:?}");
    obj.reset();
}
