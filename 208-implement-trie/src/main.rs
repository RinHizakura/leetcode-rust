struct Trie {
    key: char,
    less: Option<Box<Trie>>,
    equal: Option<Box<Trie>>,
    large: Option<Box<Trie>>,
    mark: bool,
}

impl Trie {
    fn new() -> Self {
        // create a dummy node
        Self::create('i')
    }

    fn create(c: char) -> Self {
        Trie {
            key: c,
            less: None,
            equal: None,
            large: None,
            mark: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut chars = word.chars();
        let mut ch = chars.next();

        let mut node = &mut self.equal;

        while let Some(n) = node {
            if let Some(c) = ch {
                let diff = c as i8 - n.key as i8;
                if diff > 0 {
                    node = &mut n.large;
                } else if diff < 0 {
                    node = &mut n.less;
                } else {
                    node = &mut n.equal;
                    ch = chars.next();
                }
            } else {
                n.mark = true;
                return;
            }
        }

        while let Some(c) = ch {
            *node = Some(Box::new(Self::create(c)));
            node = &mut node.as_mut().unwrap().as_mut().equal;
            ch = chars.next();
        }

        *node = Some(Box::new(Self::create('\0')));
        node.as_mut().unwrap().as_mut().mark = true;
    }

    fn traverse(&self, word: String) -> &Option<Box<Trie>> {
        let mut chars = word.chars();
        let mut ch = chars.next();

        let mut node = &self.equal;

        while let Some(c) = ch {
            if let Some(n) = node {
                let diff = c as i8 - n.key as i8;
                if diff > 0 {
                    node = &n.large;
                } else if diff < 0 {
                    node = &n.less;
                } else {
                    node = &n.equal;
                    ch = chars.next();
                }
            } else {
                break;
            }
        }

        node
    }

    fn search(&self, word: String) -> bool {
        let node = self.traverse(word);
        node.as_ref().map_or(false, |n| n.mark)
    }

    fn starts_with(&self, prefix: String) -> bool {
        let node = self.traverse(prefix);
        node.is_some()
    }
}

fn main() {
    let mut obj = Trie::new();
    obj.insert("abc".to_string());
    let ret_1: bool = obj.search("abc".to_string());
    let ret_2: bool = obj.starts_with("ab".to_string());
    println!("{} {}", ret_1, ret_2);
}
