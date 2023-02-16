struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.as_bytes();
        let mut last_pos: [i32; 26] = [-1; 26];
        for (pos, c) in s.into_iter().enumerate() {
            last_pos[(c - b'a') as usize] = pos as i32;
        }

        let mut stack: Vec<u8> = Vec::new();
        let mut bitmap: u32 = 0;
        for (pos, c) in s.into_iter().enumerate() {
            if (bitmap >> (c - b'a') & 1) == 1 {
                continue;
            }

            while !stack.is_empty() {
                let top = stack[stack.len() - 1];
                /* If this character is smaller than the one on the top stack, and
                 * there's a same character which will appear later, we can remove
                 * it. */
                if top < *c || last_pos[(top - b'a') as usize] < pos as i32 {
                    break;
                }
                bitmap &= !(1 << (top - b'a'));
                stack.pop();
            }

            stack.push(*c);
            bitmap |= 1 << (c - b'a');
        }

        std::str::from_utf8(&stack).unwrap().to_string()
    }
}

fn main() {
    println!(
        "{}",
        Solution::remove_duplicate_letters("bacbc".to_string())
    );
}
