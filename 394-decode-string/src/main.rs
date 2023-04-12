struct Solution;
enum Token {
    Num(usize),
    Pattern(String),
    LB,
    RB,
}

struct Lexer {
    s: String,
    pos: usize,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Lexer { s: s, pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let s = self.s.as_bytes();
        let len = s.len();

        while self.pos < len {
            let c = s[self.pos] as char;
            self.pos += 1;
            match c {
                '[' => return Some(Token::LB),
                ']' => return Some(Token::RB),
                _ => {
                    if c.is_ascii_digit() {
                        let mut num = c as usize - b'0' as usize;

                        while self.pos < len {
                            let c = s[self.pos] as char;
                            if c.is_ascii_digit() {
                                num = num * 10 + (c as usize - b'0' as usize);
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }

                        return Some(Token::Num(num));
                    } else {
                        let mut pat = String::new();
                        pat.push(c);

                        while self.pos < len {
                            let c = s[self.pos] as char;
                            if c.is_ascii_alphabetic() {
                                pat.push(c);
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }

                        return Some(Token::Pattern(pat));
                    }
                }
            }
        }

        None
    }
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut lexer = Lexer::new(s);

        let mut n_stack: Vec<usize> = Vec::new();
        let mut s_stack: Vec<(bool, String)> = Vec::new();

        while let Some(token) = lexer.next_token() {
            match token {
                Token::Num(num) => {
                    n_stack.push(num);
                }
                Token::Pattern(pat) => {
                    s_stack.push((true, pat));
                }
                Token::LB => {
                    s_stack.push((false, "".to_string()));
                }
                Token::RB => {
                    let n = n_stack.pop().unwrap() as usize;
                    // merge the string until the first matching left braket
                    let mut s = s_stack.pop().unwrap().1;
                    while let Some(s_pop) = s_stack.pop() {
                        s = s_pop.1 + &s;

                        if s_pop.0 == false {
                            break;
                        }
                    }

                    s_stack.push((true, s.repeat(n)));
                }
            }
        }

        /* Merge the remaining content in the stack */
        let mut s = s_stack.pop().unwrap().1;
        while let Some(s_pop) = s_stack.pop() {
            s = s_pop.1 + &s;
        }
        s
    }
}

fn main() {
    println!("{:?}", Solution::decode_string("pq4[2[2[jk]]]".to_string()));
}
