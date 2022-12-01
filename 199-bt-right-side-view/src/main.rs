use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn eval(v: &mut VecDeque<Rc<RefCell<TreeNode>>>) -> i32 {
        let l = v.len();
        let mut val = 0;

        for _ in 0..l {
            let next = v.pop_front().unwrap();
            let mut b = next.borrow_mut();
            val = b.val;
            if let Some(n) = b.left.take() {
                v.push_back(n);
            }

            if let Some(n) = b.right.take() {
                v.push_back(n);
            }
        }

        val
    }

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut v = VecDeque::new();

        if root.is_none() {
            return ans;
        }

        v.push_back(root.unwrap());
        while !v.is_empty() {
            ans.push(Self::eval(&mut v));
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
