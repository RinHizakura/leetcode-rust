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
        /* FIXME: We use value to identify the difference of node. Can we compare if two
         * reference are pointing to the same instance in Rust? */
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

        if root_val == p_val {
            return p;
        }

        if root_val == q_val {
            return q;
        }

        let mut l = None;
        let mut r = None;
        if let Some(root_node) = &root {
            l = Self::lowest_common_ancestor(
                root_node.borrow_mut().left.clone(),
                p.clone(),
                q.clone(),
            );
            r = Self::lowest_common_ancestor(root_node.borrow_mut().right.clone(), p, q);
        }

        if l.is_none() {
            r
        } else if r.is_none() {
            l
        } else {
            root.clone()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
