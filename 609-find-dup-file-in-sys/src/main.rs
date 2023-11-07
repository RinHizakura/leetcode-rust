use std::collections::HashMap;
struct Solution;
impl Solution {
    fn search(
        dir: &str,
        file: &str,
        mut next_id: usize,
        map: &mut HashMap<String, (usize, String)>,
        ans: &mut Vec<Vec<String>>,
    ) -> usize {
        let split: Vec<&str> = file.split_terminator('(').collect();
        let file = format!("{}/{}", dir, split[0]);
        let content = split[1];

        if let Some(pair) = map.get_mut(content) {
            if pair.0 == usize::MAX {
                pair.0 = next_id;
                ans.push(vec![pair.1.to_owned()]);
                next_id += 1;
            }

            ans[pair.0].push(file);
        } else {
            map.insert(content.to_owned(), (usize::MAX, file));
        }

        next_id
    }

    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut next_id = 0;
        let mut map = HashMap::new();
        let mut ans = vec![];

        for path in paths {
            let mut iter = path.split_whitespace();
            let dir = iter.next().unwrap();

            while let Some(file) = iter.next() {
                next_id = Self::search(dir, file, next_id, &mut map, &mut ans);
            }
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
            "root/c 3.txt(abcd)".to_string(),
            "root/c/d 4.txt(efgh)".to_string(),
            "root 4.txt(efgh)".to_string()
        ])
    );
}
