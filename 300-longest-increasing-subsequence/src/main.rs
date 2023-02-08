struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut v: Vec<i32> = Vec::new();

        for n in nums {
            if let Some(last) = v.last() {
                if n > *last {
                    v.push(n);
                } else {
                    let idx = v.binary_search(&n);
                    if let Err(idx) = idx {
                        v[idx] = n;
                    }
                }
            } else {
                v.push(n);
            }
        }

        v.len() as i32
    }
}

fn main() {
    println!("{}", Solution::length_of_lis(vec![1, 3, 4, 0, 1]));
}
