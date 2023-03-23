use rand::{thread_rng, Rng};

struct Solution {
    backup: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { backup: nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.backup.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut v = self.backup.clone();

        let mut len = v.len();

        while len > 0 {
            let mut rng = thread_rng();
            let x: usize = rng.gen::<usize>() % len;

            /* Swap the random select value to the last
             * stored location. */
            v.swap(x, len - 1);

            len -= 1;
        }

        v
    }
}

fn main() {
    let obj = Solution::new(vec![1, 2, 3]);
    let ret_1: Vec<i32> = obj.reset();
    let ret_2: Vec<i32> = obj.shuffle();

    println!("{:?} {:?}", ret_1, ret_2);
}
