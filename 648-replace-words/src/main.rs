#[derive(Clone)]
struct Trie {
    nodes: Vec<Option<Box<Trie>>>,
    s: Option<String>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            nodes: vec![None; 26],
            s: None,
        }
    }

    fn insert(&mut self, s: &str) {
        let mut cur = self;
        let s_arr = s.as_bytes();

        for c in s_arr {
            let idx = (c - b'a') as usize;
            if cur.nodes[idx].is_none() {
                cur.nodes[idx] = Some(Box::new(Trie::new()));
            }

            cur = cur.nodes[idx].as_mut().unwrap();
        }

        cur.s = Some(s.to_string());
    }

    fn search(&mut self, s: &str) -> Option<&String> {
        let mut cur = self;
        let s_arr = s.as_bytes();

        for c in s_arr {
            let idx = (c - b'a') as usize;

            if cur.s.is_some() {
                break;
            }

            if cur.nodes[idx].is_none() {
                return None;
            }
            cur = cur.nodes[idx].as_mut().unwrap();
        }

        return cur.s.as_ref();
    }
}

struct Solution;
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for word in dictionary {
            trie.insert(&word);
        }

        let mut new = String::new();
        let v: Vec<&str> = sentence.split(' ').collect();

        for word in v {
            if let Some(root) = trie.search(word) {
                new.push_str(root);
            } else {
                new.push_str(word);
            }
            new.push(' ');
        }
        // Pop out the last space
        new.pop();

        new
    }
}

fn main() {
    println!(
        "{}",
        Solution::replace_words(
            vec!["a".to_string(), "b".to_string()],
            "aa bb cc".to_string()
        )
    );
}
