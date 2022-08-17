use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.into_iter() {
            let mut sort_s: Vec<char> = s.chars().collect();
            sort_s.sort_by(|a, b| b.cmp(a));

            let key = sort_s.iter().collect();

            match map.entry(key) {
                Entry::Vacant(e) => {
                    e.insert(vec![s]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(s);
                }
            }
        }

        let mut ans = Vec::new();

        for (_, v) in map.drain() {
            ans.push(v);
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(vec!["ate".to_string(), "tea".to_string()])
    );
}
