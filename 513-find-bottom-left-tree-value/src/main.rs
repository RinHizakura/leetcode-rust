use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        let mut val = 0;

        q.push_back(root.unwrap());

        while !q.is_empty() {
            let len = q.len();

            for idx in 0..len {
                if let Some(n) = q.pop_front() {
                    if idx == 0 {
                        val = n.borrow().val;
                    }

                    let left = n.borrow().left.clone();
                    if left.is_some() {
                        q.push_back(left.unwrap());
                    }

                    let right = n.borrow().right.clone();
                    if right.is_some() {
                        q.push_back(right.unwrap());
                    }
                }
            }
        }

        val
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{}", Solution::find_bottom_left_value(root));
}
