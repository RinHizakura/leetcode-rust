use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn rob_recursive(node: Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
        if node.is_none() {
            return [0, 0];
        }

        let rob_left = Self::rob_recursive(node.as_ref().unwrap().borrow().left.clone());
        let rob_right = Self::rob_recursive(node.as_ref().unwrap().borrow().right.clone());

        let mut ans = [0, 0];
        // ans[0] means that we don't rob the current node
        ans[0] = rob_left[0].max(rob_left[1]) + rob_right[0].max(rob_right[1]);
        // ans[1] means that we rob the current node
        ans[1] = node.unwrap().borrow().val + rob_left[0] + rob_right[0];

        ans
    }

    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        *Self::rob_recursive(root).iter().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
