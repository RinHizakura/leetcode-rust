use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut q = VecDeque::new();
        let mut ans = vec![];

        if root.is_none() {
            return ans;
        }

        q.push_back(root.unwrap());

        while !q.is_empty() {
            let len = q.len();
            let mut val = i32::MIN;

            for _ in 0..len {
                if let Some(n) = q.pop_front() {
                    val = val.max(n.borrow().val);

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

            ans.push(val);
        }

        ans
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{:?}", Solution::largest_values(root));
}
