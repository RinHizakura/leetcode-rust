struct Solution;
impl Solution {

    pub fn recursive_solver(s: &mut String, v: &mut Vec<String>, left: i32, right:i32, n: i32) {
        if left + right == n * 2 {
            v.push(s.clone());
            return;
        }

        if left < n {
            s.push('(');
            Solution::recursive_solver(s, v, left + 1, right, n);
            s.pop();
        }

        if left > right {
            s.push(')');
            Solution::recursive_solver(s, v, left, right+1, n);
            s.pop();
        }

    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut v = Vec::new();
        let mut s = String::new();

        Solution::recursive_solver(&mut s, &mut v, 0, 0, n);

        v
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
