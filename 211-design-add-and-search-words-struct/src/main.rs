#[derive(Clone)]
struct Trie {
    mark_end: bool,
    child: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn default() -> Self {
        const NONE: Option<Box<Trie>> = None;
        Trie {
            mark_end: false,
            child: [NONE; 26],
        }
    }

    fn add(&mut self, c: usize) -> &mut Option<Box<Trie>> {
        if self.child[c].is_none() {
            self.child[c] = Some(Box::new(Trie::default()));
        }

        &mut self.child[c]
    }

    fn find(&self, c: usize) -> &Option<Box<Trie>> {
        &self.child[c]
    }
}

struct WordDictionary {
    root: Trie,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: Trie::default(),
        }
    }

    fn add_word(&mut self, word: String) {
        let len = word.len();
        let word = word.as_bytes();
        let mut child = self.root.add((word[0] - 'a' as u8) as usize);

        for i in 1..len {
            if let Some(tmp) = child {
                child = tmp.add((word[i] - 'a' as u8) as usize);
            }
        }

        if let Some(tmp) = child {
            tmp.mark_end = true;
        }
    }

    fn search(&self, word: String) -> bool {
        let len = word.len();
        let word = word.as_bytes();

        Self::__search(&self.root, word, 0, len)
    }

    fn __search(trie: &Trie, word: &[u8], i: usize, len: usize) -> bool {
        if i >= len {
            return trie.mark_end;
        }

        if word[i] == '.' as u8 {
            for j in 0..26 {
                if let Some(next_trie) = trie.find(j) {
                    if Self::__search(next_trie, word, i + 1, len) == true {
                        return true;
                    }
                }
            }

            return false;
        }

        if let Some(next_trie) = trie.find((word[i] - 'a' as u8) as usize) {
            Self::__search(next_trie, word, i + 1, len)
        } else {
            false
        }
    }
}

fn main() {
    let mut obj = WordDictionary::new();
    obj.add_word("abc".to_string());
    let ret: bool = obj.search("abc".to_string());
    println!("{}", ret);
}
