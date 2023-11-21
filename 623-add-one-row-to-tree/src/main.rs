use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn walk(node: &mut Option<Rc<RefCell<TreeNode>>>, cur: i32, depth: i32, val: i32) {
        if let Some(n) = node {
            if cur == depth {
                let mut borrow = n.borrow_mut();

                let left = &mut borrow.left;
                let n_l = if left.is_some() {
                    TreeNode {
                        val,
                        left: left.clone(),
                        right: None,
                    }
                } else {
                    TreeNode::new(val)
                };
                *left = Some(Rc::new(RefCell::new(n_l)));

                let right = &mut borrow.right;
                let n_r = if right.is_some() {
                    TreeNode {
                        val,
                        left: None,
                        right: right.clone(),
                    }
                } else {
                    TreeNode::new(val)
                };
                *right = Some(Rc::new(RefCell::new(n_r)));
                return;
            }

            Self::walk(&mut n.borrow().left.clone(), cur + 1, depth, val);
            Self::walk(&mut n.borrow().right.clone(), cur + 1, depth, val);
        }
    }

    pub fn add_one_row(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }

        Self::walk(&mut root, 1, depth - 1, val);

        root
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{:?}", Solution::add_one_row(root, 10, 1));
}
