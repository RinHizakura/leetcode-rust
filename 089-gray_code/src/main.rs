struct Solution;
impl Solution {
    pub fn recursive_solver(n: i32, i: i32, v: &mut Vec<i32>) {
        if n == i {
            return;
        }

        let start = 1 << i;
        let end = 1 << (i + 1);
        for idx in start..end {
            let tmp = v[(end - 1 - idx) as usize];
            v.push(tmp | start);
        }
        Solution::recursive_solver(n, i + 1, v)
    }

    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 1 {
            return vec![0, 1];
        }

        let mut v = vec![0, 1];
        Solution::recursive_solver(n, 1, &mut v);
        v
    }
}

fn main() {
    println!("{:?}", Solution::gray_code(3));
}
