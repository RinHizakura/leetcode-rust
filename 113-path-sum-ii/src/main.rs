use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn recursive(
        node: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        match node {
            Some(n) => {
                let val = n.borrow().val;
                tmp.push(val);

                if n.borrow().left.is_none() && n.borrow().right.is_none() {
                    if val == target_sum {
                        ans.push(tmp.clone());
                    }
                } else {
                    Solution::recursive(n.borrow().left.clone(), target_sum - val, tmp, ans);
                    Solution::recursive(n.borrow().right.clone(), target_sum - val, tmp, ans);
                }
                tmp.pop();
            }
            None => {}
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut tmp = vec![];
        let mut ans = vec![];
        Solution::recursive(root, target_sum, &mut tmp, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
