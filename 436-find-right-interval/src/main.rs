use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_right_interval(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();

        for (idx, i) in intervals.iter().enumerate() {
            map.insert(i[0], idx);
        }

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let len = intervals.len();
        let mut ans = vec![-1; len];

        for i in intervals.iter() {
            let key = i[1];
            let orig_idx = *map.get(&i[0]).unwrap();

            let pos = intervals.binary_search_by(|a| a[0].cmp(&key));

            let tmp = pos.unwrap_or_else(|err| err);

            if tmp >= len {
                continue;
            }

            ans[orig_idx] = *map.get(&intervals[tmp][0]).unwrap() as i32;
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::find_right_interval(vec![vec![1,2], vec![3,4]]));
}
