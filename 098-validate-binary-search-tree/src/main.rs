use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn valid(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(n) => {
                let val = n.borrow().val as i64;
                if val <= min || val >= max {
                    false
                } else {
                    Solution::valid(n.borrow().left.clone(), min, val)
                        && Solution::valid(n.borrow().right.clone(), val, max)
                }
            }
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::valid(root, i64::MIN, i64::MAX)
    }
}

fn main() {
    println!("");
}
