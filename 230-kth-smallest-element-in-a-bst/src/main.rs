use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) -> i32 {
        if let Some(node) = node {
            Self::traverse(&node.borrow_mut().left, v);
            v.push(node.borrow_mut().val);
            Self::traverse(&node.borrow_mut().right, v);
        }

        return -1;
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut v = vec![];
        Self::traverse(&root, &mut v);
        v[k as usize - 1]
    }
}

fn main() {
    println!("Hello, world!");
}
