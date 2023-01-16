use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_val;
        if let Some(root_node) = &root {
            root_val = root_node.borrow_mut().val;
        } else {
            return None;
        }

        let p_val;
        if let Some(p_node) = &p {
            p_val = p_node.borrow_mut().val;
        } else {
            unreachable!();
        }

        let q_val;
        if let Some(q_node) = &q {
            q_val = q_node.borrow_mut().val;
        } else {
            unreachable!();
        }

        if q_val > root_val && p_val > root_val {
            Self::lowest_common_ancestor(root.unwrap().borrow_mut().right.clone(), p, q)
        } else if q_val < root_val && p_val < root_val {
            Self::lowest_common_ancestor(root.unwrap().borrow_mut().left.clone(), p, q)
        } else {
            root
        }
    }
}

fn main() {
    println!("Hello, world!");
}
