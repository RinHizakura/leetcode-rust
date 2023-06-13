use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_string();
        }

        let mut s = String::new();
        let mut v = VecDeque::new();

        v.push_back(root);

        while !v.is_empty() {
            let node = v.pop_front().unwrap();

            if let Some(n) = node {
                let val = n.borrow().val;
                s.push_str(&val.to_string());
                s.push(',');

                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();
                v.push_back(left);
                v.push_back(right);
            } else {
                s.push('*');
                s.push(',');
            }
        }
        s
    }

    fn parse_next(data: &[u8], start: usize, len: usize) -> Option<(usize, i32)> {
        let mut n = 0;
        for idx in start..len {
            let c = data[idx];
            if c == b',' {
                return Some((idx + 1, n));
            } else if c == b'*' {
                n = i32::MAX;
            } else {
                n = n * 10 + (c - b'0') as i32;
            }
        }

        return None;
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }

        let data = data.as_bytes();
        let len = data.len();

        let (mut idx, val) = Self::parse_next(data, 0, len).unwrap();

        let mut v = VecDeque::new();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        v.push_back(root.clone());
        v.push_back(root.clone());

        let mut is_right = 0;
        while let Some((i, val)) = Self::parse_next(data, idx, len) {
            let top = v.pop_front().unwrap();

            let node = if val != i32::MAX {
                Some(Rc::new(RefCell::new(TreeNode::new(val))))
            } else {
                None
            };

            if node.is_some() {
                v.push_back(node.clone());
                v.push_back(node.clone());
            }

            if (is_right & 1) == 0 {
                top.unwrap().borrow_mut().left = node;
            } else {
                top.unwrap().borrow_mut().right = node;
            }
            is_right ^= 1;

            idx = i;
        }

        root
    }
}

fn main() {
    let obj = Codec::new();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let data: String = obj.serialize(root);
    println!("{}", data);
    let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
    println!("{:?}", ans);
}
