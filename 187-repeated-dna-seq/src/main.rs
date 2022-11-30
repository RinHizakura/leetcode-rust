use std::collections::HashMap;

struct Solution;
impl Solution {
    fn encode(c: char) -> u64 {
        match c {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => unreachable!(),
        }
    }

    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut v = vec![];
        let len = s.len();

        if len < 10 {
            return v;
        }

        let mut chars = s.chars();
        let mut code: u64 = 0;
        for _ in 0..10 {
            code <<= 2;
            code |= Self::encode(chars.next().unwrap());
        }
        code &= 0xfffff;

        let mut i = 1;
        let mut map = HashMap::new();
        map.insert(code, 1);

        while let Some(c) = chars.next() {
            code <<= 2;
            code |= Self::encode(c);
            code &= 0xfffff;

            if map.contains_key(&code) {
                if let Some(x) = map.get_mut(&code) {
                    if *x == 1 {
                        *x = 2;
                        v.push(s[i..i + 10].to_string());
                    }
                }
            } else {
                map.insert(code, 1);
            }

            i += 1;
        }

        v
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string())
    );
}
