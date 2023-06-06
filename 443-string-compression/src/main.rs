struct Solution;
impl Solution {
    fn update(chars: &mut Vec<char>, c: char, write: &mut usize, count: usize) {
        chars[*write] = c;
        *write += 1;

        if count == 1 {
            return;
        }

        for c in count.to_string().chars() {
            chars[*write] = c;
            *write += 1;
        }
    }

    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let len = chars.len();

        let mut write = 0;
        let mut read = 1;
        let mut count = 1;
        let mut cur = chars[0];

        while read < len {
            if cur != chars[read] {
                Self::update(chars, cur, &mut write, count);
                cur = chars[read];
                count = 1;
            } else {
                count += 1;
            }

            read += 1;
        }
        Self::update(chars, cur, &mut write, count);

        write as i32
    }
}

fn main() {
    let mut v = vec!['a', 'a', 'b'];
    let ret = Solution::compress(&mut v);
    println!("{:?} {}", v, ret);
}
