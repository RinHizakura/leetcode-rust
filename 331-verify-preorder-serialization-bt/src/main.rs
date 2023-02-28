struct Parser {
    preorder: Vec<u8>,
    idx: usize,
    len: usize,
}

impl Parser {
    pub fn next(&mut self) {
        while self.idx < self.len && self.preorder[self.idx] != b',' {
            self.idx += 1;
        }
        // add one to move to the next node
        self.idx += 1;
    }

    pub fn current(&self) -> Option<usize> {
        if self.idx >= self.len {
            None
        } else if self.preorder[self.idx] == b'#' {
            Some(0)
        } else {
            Some(1)
        }
    }
}

struct Solution;
impl Solution {
    fn is_valid(parser: &mut Parser) -> bool {
        // move to the (possible) left node
        parser.next();
        let result_left = match parser.current() {
            Some(0) => true,
            Some(1) => Self::is_valid(parser),
            None => false,
            Some(_) => unreachable!(),
        };

        if result_left == false {
            return false;
        }

        // move to the possible right node
        parser.next();
        let result_right = match parser.current() {
            Some(0) => true,
            Some(1) => Self::is_valid(parser),
            None => false,
            Some(_) => unreachable!(),
        };

        result_right
    }

    pub fn is_valid_serialization(preorder: String) -> bool {
        let preorder = preorder.as_bytes();
        let len = preorder.len();

        if preorder[0] == b'#' {
            if len == 1 {
                return true;
            }
            return false;
        }

        let mut parser = Parser {
            preorder: preorder.to_vec(),
            idx: 0,
            len: len,
        };

        let result = Self::is_valid(&mut parser);

        // We should make share that the string is parsed fully
        if result == false || parser.idx + 1 != len {
            return false;
        }

        true
    }
}

fn main() {
    println!("{}", Solution::is_valid_serialization("#,#".to_string()));
}
