struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut total = 0;
        let mut cur = 0;
        let mut start = 0;

        for i in 0..len {
            let to_next = gas[i] - cost[i];
            total += to_next;
            cur += to_next;

            if cur < 0 {
                cur = 0;
                start = i + 1;
            }
        }

        if total >= 0 {
            return start as i32;
        }

        return -1;
    }
}

fn main() {
    println!("{}", Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]));
}
