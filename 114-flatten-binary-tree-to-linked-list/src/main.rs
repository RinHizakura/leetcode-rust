use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut cur: Option<Rc<RefCell<TreeNode>>> = root.clone();

        while let Some(n) = cur {
            let mut n = n.borrow_mut();
            // if the left subtree exist
            if let Some(mut target) = n.left.clone() {
                // find the rightmost leaf of the left subtree
                loop {
                    let right = target.borrow().right.clone();
                    if let Some(r) = right {
                        target = r;
                    } else {
                        break;
                    }
                }

                // attach the right subtree to the rightmost leaf
                target.borrow_mut().right = n.right.take();
                // attach the left subtree to right
                n.right = n.left.take();
            }

            cur = n.right.clone();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
