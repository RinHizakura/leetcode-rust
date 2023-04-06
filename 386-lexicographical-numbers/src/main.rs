struct Solution;
impl Solution {
    pub fn recursive(next: i32, n: i32, v: &mut Vec<i32>) {
        let range;
        if next == 1 {
            range = 0..=8;
        } else {
            range = 0..=9;
        }

        for i in range {
            let tmp = next + i;
            if tmp > n {
                return;
            }
            v.push(tmp);
            Self::recursive(tmp * 10, n, v);
        }
    }

    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut v = vec![];
        Self::recursive(1, n, &mut v);
        v
    }
}

fn main() {
    println!("{:?}", Solution::lexical_order(100));
}
