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
    pub fn calculate(s: String) -> i32 {
        let mut lexer = Lexer::new(s);

        let mut i_stack: Vec<i64> = Vec::new();
        let mut sign = 1;

        while let Some(token) = lexer.next_token() {
            match token {
                Token::I(i) => {
                    i_stack.push(sign * i);
                }
                Token::Op(op) => match op {
                    '*' => {
                        let i = i_stack.pop().unwrap();
                        if let Token::I(i2) = lexer.next_token().unwrap() {
                            i_stack.push(i * i2);
                        }
                    }
                    '/' => {
                        let i = i_stack.pop().unwrap();
                        if let Token::I(i2) = lexer.next_token().unwrap() {
                            i_stack.push(i / i2);
                        }
                    }
                    '-' => {
                        sign = -1;
                    }
                    '+' => {
                        sign = 1;
                    }
                    _ => unreachable!(),
                },
            }
        }

        let mut num = i_stack.pop().unwrap();
        while let Some(i) = i_stack.pop() {
            num += i;
        }

        num as i32
    }
}

fn main() {
    println!("{}", Solution::calculate("1-1+1".to_string()));
}
