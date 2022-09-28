use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn recursive(left: i32, right: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if left > right {
            return vec![None];
        } else if left == right {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(left))))];
        }

        let mut v = vec![];

        for i in left..=right {
            let lefts = Solution::recursive(left, i - 1);
            let rights = Solution::recursive(i + 1, right);
            for l in lefts.iter() {
                for r in rights.iter() {
                    let mut root = Rc::new(RefCell::new(TreeNode::new(i)));
                    root.borrow_mut().left = l.clone();
                    root.borrow_mut().right = r.clone();
                    v.push(Some(root));
                }
            }
        }

        v
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Solution::recursive(1, n)
    }
}

fn main() {
    let v = Solution::generate_trees(3);
    for i in v {
        println!("{:?}\n", i);
    }
}
