struct Solution;
impl Solution {
    fn recursive(tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, i: i32, n: i32) {
        if i >= n {
            return;
        }
        tmp.push(1);
        for idx in (1..(i as usize)).rev() {
            tmp[idx] = tmp[idx - 1] + tmp[idx];
        }
        ans.push(tmp.clone());

        Solution::recursive(tmp, ans, i + 1, n);
    }

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut tmp = vec![];
        let mut ans = vec![];
        Solution::recursive(&mut tmp, &mut ans, 0, num_rows);

        ans
    }
}

fn main() {
    println!("{:?}", Solution::generate(5));
}
