use rand::prelude::*;

struct Solution {
    ws: Vec<i32>,
    len: i32,
}

impl Solution {
    fn new(mut ws: Vec<i32>) -> Self {
        let len = ws.len();

        for idx in 1..len {
            ws[idx] += ws[idx - 1];
        }

        let len = len as i32;
        Solution { ws, len }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let len = self.len as usize;
        let r: u64 = rng.gen::<u64>() % self.ws[len - 1] as u64;
        let r = r as u32 as i32;

        let idx = self.ws.binary_search(&r);

        if let Ok(i) = idx {
            return i as i32 + 1;
        }

        if let Err(i) = idx {
            return i as i32;
        }

        unreachable!()
    }
}

fn main() {
    let obj = Solution::new(vec![1, 2, 5]);
    let ret_1: i32 = obj.pick_index();
    println!("{}", ret_1);
}
