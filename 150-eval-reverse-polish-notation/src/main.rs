struct Solution;
impl Solution {
    fn is_operator(ch: char) -> bool {
        if (ch as u8) < ('0' as u8) {
            true
        } else {
            false
        }
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            let l = token.len();
            let ch = token.chars().next().unwrap();
            if l == 1 && Self::is_operator(ch) {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                match token.chars().next().unwrap() {
                    '+' => stack.push(n1 + n2),
                    '-' => stack.push(n1 - n2),
                    '*' => stack.push(n1 * n2),
                    '/' => stack.push(n1 / n2),
                    _ => unreachable!(),
                }
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }

        stack.pop().unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string()])
    );
}
