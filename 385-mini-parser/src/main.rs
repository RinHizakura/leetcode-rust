struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

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

        let mut sign = 1;
        while self.pos < self.len {
            let c = s[self.pos] as char;
            self.pos += 1;
            match c {
                '[' | ']' | ',' => return Some(Token::Op(c)),
                '-' => sign = -1,
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

                    return Some(Token::I(num * sign));
                }
            }
        }

        None
    }
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut lexer = Lexer::new(s);

        let mut prev: Vec<Option<NestedInteger>> = vec![];
        let mut cur: Option<NestedInteger> = None;

        while let Some(token) = lexer.next_token() {
            match token {
                Token::I(n) => {
                    if let Some(c) = &mut cur {
                        if let NestedInteger::List(v) = c {
                            v.push(NestedInteger::Int(n as i32));
                        }
                    } else {
                        cur = Some(NestedInteger::Int(n as i32));
                    }
                }
                Token::Op(op) => {
                    match op {
                        '[' => {
                            prev.push(cur.take());
                            cur = Some(NestedInteger::List(vec![]));
                        }
                        ']' => {
                            let mut top = prev.pop().unwrap();
                            if let Some(t) = &mut top {
                                if let NestedInteger::List(v) = t {
                                    v.push(cur.take().unwrap());
                                }
                                /* Restore cur if the top is not None, so we can
                                 * simply get the last NestedInteger which should be the
                                 * answer */
                                cur = top;
                            }
                        }
                        ',' => {}
                        _ => unreachable!(),
                    }
                }
            }
        }

        cur.unwrap()
    }
}

fn main() {
    println!("{:?}", Solution::deserialize("123".to_string()));
}
