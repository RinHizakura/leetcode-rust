struct Solution;
impl Solution {
    fn distance(a: u32, b: u32) -> usize {
        ((a ^ b).count_ones() >> 1) as usize
    }

    fn is_visited(mask: usize, idx: usize) -> bool {
        ((mask >> idx) & 1) == 1
    }

    fn set_visited(mask: &mut usize, idx: usize) {
        *mask |= 1 << idx;
    }

    fn str_to_mask(s: &String) -> u32 {
        let mut mask: u32 = 0;
        for (idx, c) in s.chars().enumerate() {
            match c {
                'A' => mask |= 0b0001 << (idx * 4),
                'C' => mask |= 0b0010 << (idx * 4),
                'G' => mask |= 0b0100 << (idx * 4),
                'T' => mask |= 0b1000 << (idx * 4),
                _ => unreachable!(),
            }
        }
        mask
    }

    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        /* We could convert every genes to a bitmask, this help
         * us to calculate distance between them. */
        let len = bank.len();
        let mut v = Vec::with_capacity(len + 1);
        let end_mask = Self::str_to_mask(&end_gene);
        let mut end = len + 1;
        for (idx, s) in bank.iter().enumerate() {
            let mask = Self::str_to_mask(s);
            if end_mask == mask {
                end = idx;
            }
            v.push(mask);
        }
        v.push(Self::str_to_mask(&start_gene));

        if end == len + 1 {
            return -1;
        }

        let mut step = 0;

        let mut prev_visited = 1 << len;
        let mut visited = prev_visited;

        while (prev_visited != 0) && !Self::is_visited(visited, end) {
            let mut new_visited = 0;

            for i in 0..=len {
                if Self::is_visited(prev_visited, i) {
                    for j in 0..len {
                        if !Self::is_visited(visited, j) && Self::distance(v[i], v[j]) == 1 {
                            Self::set_visited(&mut new_visited, j);
                            Self::set_visited(&mut visited, j);
                        }
                    }
                }
            }

            prev_visited = new_visited;
            step += 1;
        }

        if !Self::is_visited(visited, end) {
            return -1;
        }

        step
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AACCGGTA".to_string(),
            vec!["AACCGGTA".to_string()]
        )
    );
}
