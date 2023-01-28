struct Solution;

enum Token {
    I(i64),
    Op(char),
}

struct Lexer {
    s: String,
    pos: usize,
    len: usize,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        let l = s.len();
        Lexer {
            s: s,
            pos: 0,
            len: l,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let s = self.s.as_bytes();

        while self.pos < self.len {
            let c = s[self.pos] as char;
            self.pos += 1;
            match c {
                '+' | '-' | '*' | '/' => return Some(Token::Op(c)),
                ' ' => {}
                _ => {
                    /* should be digit */
                    let mut num: i64 = c as i64 - '0' as i64;

                    while self.pos < self.len {
                        let c = s[self.pos];
                        if c.is_ascii_digit() {
                            num = num * 10 + (c as i64 - '0' as i64);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }

                    return Some(Token::I(num));
                }
            }
        }

        None
    }
}

impl Solution {
    fn solver(i_stack: &Vec<i32>, op_stack: &Vec<char>, start: usize, end: usize) -> Vec<i32> {
        let mut result = vec![];
        for i in start..end {
            let result_left = Self::solver(i_stack, op_stack, start, i);
            let result_right = Self::solver(i_stack, op_stack, i + 1, end);

            let f = match op_stack[i] {
                '+' => |a, b| a + b,
                '-' => |a, b| a - b,
                '*' => |a, b| a * b,
                _ => unreachable!(),
            };

            for l in &result_left {
                for r in &result_right {
                    result.push(f(l, r));
                }
            }
        }

        if result.is_empty() {
            result.push(i_stack[start]);
        }

        result
    }

    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut lexer = Lexer::new(expression);

        let mut op_stack = Vec::new();
        let mut i_stack = Vec::new();

        while let Some(token) = lexer.next_token() {
            match token {
                Token::I(i) => i_stack.push(i as i32),
                Token::Op(op) => op_stack.push(op),
            }
        }

        Self::solver(&i_stack, &op_stack, 0, i_stack.len() - 1)
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::diff_ways_to_compute("2*3-4*5".to_string())
    );
}
