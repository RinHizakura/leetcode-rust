struct Solution;
impl Solution {
    pub fn recursive(k: i32, n: i32, max_num: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if k > n {
            return;
        }

        if k == 0 && n == 0 {
            ans.push(tmp.clone());
            return;
        }

        for i in 1..max_num {
            tmp.push(i);
            Self::recursive(k - 1, n - i, i, tmp, ans);
            tmp.pop();
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        if n > 45 {
            return ans;
        }

        let mut tmp = vec![];
        Self::recursive(k, n, n.min(10), &mut tmp, &mut ans);
        ans
    }
}

fn main() {
    println!("{:?}", Solution::combination_sum3(3, 7));
}
