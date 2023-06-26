struct Solution;
impl Solution {
    fn map(c: u8) -> usize {
        let ans = if c <= b'9' {
            c - b'0'
        } else if c <= b'Z' {
            10 + (c - b'A')
        } else {
            10 + 26 + (c - b'a')
        };

        ans as usize
    }

    fn inv_map(idx: usize) -> u8 {
        let idx = idx as u8;

        if idx < 10 {
            return b'0' + idx;
        } else if idx < 36 {
            return b'A' + (idx - 10);
        } else {
            return b'a' + (idx - 10 - 26);
        }
    }

    pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
        let mut indices = (0..data.len()).collect::<Vec<usize>>();
        indices.sort_by(|&a, &b| data[b].cmp(&data[a]));
        indices
    }

    pub fn frequency_sort(s: String) -> String {
        let mut v: Vec<i32> = vec![0; 10 + 26 + 26];
        let s = s.as_bytes();

        for c in s {
            let i = Self::map(*c);
            v[i] += 1;
        }

        let indices = Self::argsort(&v);

        let mut ans = String::new();
        for index in indices {
            if v[index] == 0 {
                break;
            }
            let c = Self::inv_map(index);
            let tmp = vec![c];
            let tmp_str = std::str::from_utf8(&tmp).unwrap();
            ans.push_str(&tmp_str.repeat(v[index] as usize));
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::frequency_sort("tree".to_string()));
}
