use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct RandomizedSet {
    slot: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            slot: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.slot.push(val);
        /* assign the index for the input value */
        self.map.insert(val, self.slot.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        /* Use hashmap to find the original index
         * of that value */
        let idx;
        if let Some(i) = self.map.get(&val) {
            idx = *i;
        } else {
            return false;
        }

        let last_idx = self.slot.len() - 1;
        /* Take the element in the last first. If it is not
         * the removed one,  we'll put it back with the new slot index */
        let last_val = self.slot[last_idx];

        self.map.remove(&val);
        self.slot.pop();

        if last_idx != idx {
            self.slot[idx] = last_val;
            self.map.remove(&last_val);
            self.map.insert(last_val, idx);
        }

        true
    }

    fn get_random(&self) -> i32 {
        let len = self.slot.len();

        let mut rng = thread_rng();
        let x: usize = rng.gen();
        self.slot[x % len]
    }
}

fn main() {
    let mut obj = RandomizedSet::new();
    let ret_1: bool = obj.insert(10);
    let ret_2: bool = obj.remove(10);
    let ret_3: bool = obj.insert(5);
    let ret_4: i32 = obj.get_random();
    println!("{} {} {} {}", ret_1, ret_2, ret_3, ret_4);
}
