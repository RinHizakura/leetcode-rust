struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.chars();
        let mut v = Vec::new();

        for c in s {
            match c {
                '(' | '{' | '[' => v.push(c),
                ')' => {
                    let top = v.pop();
                    if top.is_none() || top.unwrap() != '(' {
                        return false;
                    }
                }
                '}' => {
                    let top = v.pop();
                    if top.is_none() || top.unwrap() != '{' {
                        return false;
                    }
                }
                ']' => {
                    let top = v.pop();
                    if top.is_none() || top.unwrap() != '[' {
                        return false;
                    }
                }
                _ => return false
            }
        }

        v.is_empty()
    }
}

fn main() {
    println!("{}", Solution::is_valid("([)]".to_string()));
}
