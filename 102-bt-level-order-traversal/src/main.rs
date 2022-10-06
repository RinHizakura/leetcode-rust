use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn recursive(node: Option<Rc<RefCell<TreeNode>>>, i: usize, ans: &mut Vec<Vec<i32>>) {
        match node {
            None => return,
            Some(n) => {
                while i >= ans.len() {
                    ans.push(vec![]);
                }
                let val = n.borrow().val;
                ans[i].push(val);
                Solution::recursive(n.borrow().left.clone(), i + 1, ans);
                Solution::recursive(n.borrow().right.clone(), i + 1, ans);
            }
        };
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Solution::recursive(root, 0, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
