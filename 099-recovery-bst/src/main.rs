use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn traverse_by_order(
        node: &Option<Rc<RefCell<TreeNode>>>,
        prev_node: &mut Option<Rc<RefCell<TreeNode>>>,
        first_err: &mut Option<Rc<RefCell<TreeNode>>>,
        second_err: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        match node {
            None => return,
            Some(n) => {
                Solution::traverse_by_order(&n.borrow().left, prev_node, first_err, second_err);
                let val = n.borrow().val;
                // initialize to make compiler happy
                let mut prev_val = 0;
                match prev_node {
                    None => assert!(false),
                    Some(n_prev) => prev_val = n_prev.borrow().val,
                };
                if val < prev_val {
                    if first_err.is_none() {
                        *first_err = prev_node.clone();
                    }
                    *second_err = node.clone();
                }
                *prev_node = node.clone();
                Solution::traverse_by_order(&n.borrow().right, prev_node, first_err, second_err);
            }
        }
    }

    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut dummy = Some(Rc::new(RefCell::new(TreeNode::new(i32::MIN))));
        let mut first_err = None;
        let mut second_err = None;
        Solution::traverse_by_order(root, &mut dummy, &mut first_err, &mut second_err);
        if let (Some(mut first), Some(mut second)) = (first_err, second_err) {
            let first_val = first.borrow().val;
            first.borrow_mut().val = second.borrow().val;
            second.borrow_mut().val = first_val;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
