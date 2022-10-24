use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn to_leaf_num(node: &Option<Rc<RefCell<TreeNode>>>, tmp: &mut i32, ans: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            *tmp = *tmp * 10 + n.val;
            // leaf
            if n.left.is_none() && n.right.is_none() {
                *ans += *tmp;
            }
            Self::to_leaf_num(&n.left, tmp, ans);
            Self::to_leaf_num(&n.right, tmp, ans);
            *tmp = *tmp / 10;
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tmp = 0;
        let mut ans = 0;
        Solution::to_leaf_num(&root, &mut tmp, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
