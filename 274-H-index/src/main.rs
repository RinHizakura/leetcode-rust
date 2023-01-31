struct Solution;
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by(|a, b| b.cmp(a));

        /* This is an impossible h according to the constraint
         * on this question */
        let mut h = 1001;
        let mut count = 0;
        for cite in citations {
            if cite != h {
                if count >= h {
                    return h;
                }

                if count > cite {
                    return count;
                }
                h = cite;
            }
            count += 1;
        }

        return h.min(count);
    }
}

fn main() {
    println!("{}", Solution::h_index(vec![4, 4, 0, 0]));
}
