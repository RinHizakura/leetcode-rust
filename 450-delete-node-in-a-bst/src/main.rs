struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

impl Solution {
    pub fn merge(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left.is_none() {
            return right;
        }

        if right.is_none() {
            return left;
        }

        let mut cur = right.clone();

        while let Some(c) = cur {
            if c.borrow_mut().left.is_none() {
                c.borrow_mut().left = left;
                break;
            }

            cur = c.borrow_mut().left.to_owned();
        }

        right
    }

    pub fn recursive(node: &mut Option<Rc<RefCell<TreeNode>>>, key: i32) {
        if let Some(n) = node {
            let val = n.borrow().val;

            if val == key {
                let left = n.borrow_mut().left.take();
                let right = n.borrow_mut().right.take();

                let merge = Self::merge(left, right);

                if let Some(m) = merge {
                    n.swap(&m);
                } else {
                    *node = None;
                }
                return;
            }

            Self::recursive(&mut n.borrow_mut().left, key);
            Self::recursive(&mut n.borrow_mut().right, key);
        }
    }

    pub fn delete_node(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recursive(&mut root, key);
        root
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    println!("{:?}", Solution::delete_node(root, 0));
}
