use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn tree_leftdepth(mut node: Option<Rc<RefCell<TreeNode>>>) -> u32 {
        let mut depth = 0;
        while let Some(n) = node {
            depth += 1;
            node = n.borrow_mut().left.clone();
        }

        depth
    }

    fn tree_rightdepth(mut node: Option<Rc<RefCell<TreeNode>>>) -> u32 {
        let mut depth = 0;
        while let Some(n) = node {
            depth += 1;
            node = n.borrow_mut().right.clone();
        }

        depth
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        if let Some(root) = root {
            let mut root = root.borrow_mut();
            let left = root.left.take();
            let right = root.right.take();
            /* FIXME: Clone for the ownership model, any better approach? */
            let left_depth = Self::tree_leftdepth(left.clone());
            let right_depth = Self::tree_rightdepth(right.clone());

            if left_depth == right_depth {
                return 2_u32.pow(left_depth + 1) as i32 - 1;
            }

            return 1 + Self::count_nodes(left) + Self::count_nodes(right);
        }

        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
