use std::collections::{BinaryHeap, VecDeque};

struct Solution;
impl Solution {
    pub fn least_interval(tasks: Vec<char>, mut n: i32) -> i32 {
        let mut total = tasks.len();
        let mut task_cnt = vec![0; 26];
        let mut heap = BinaryHeap::new();

        for task in tasks {
            task_cnt[(task as u8 - b'A') as usize] += 1;
        }

        for idx in 0..26 {
            if task_cnt[idx] > 0 {
                heap.push((task_cnt[idx], idx));
            }
        }

        let mut t = 0;
        let mut wait_q = VecDeque::new();

        while total > 0 {
            t += 1;

            /* Pick the task with most count. The index 26 is a dummy
             * index which indicates idle */
            let mut idx = 26;
            if let Some(ent) = heap.pop() {
                idx = ent.1;
                task_cnt[idx] -= 1;
                total -= 1;
            }

            wait_q.push_back(idx);

            if n > 0 {
                // wait until the cooldown time
                n -= 1;
            } else {
                let wait = wait_q.pop_front().unwrap();
                // The cooldown time is finish
                if wait < 26 && task_cnt[wait] > 0 {
                    heap.push((task_cnt[wait], wait));
                }
            }
        }

        t
    }
}

fn main() {
    println!("{}", Solution::least_interval(vec!['A', 'A', 'B'], 1));
}
