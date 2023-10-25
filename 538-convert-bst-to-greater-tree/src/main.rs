use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn go(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = node {
            let n = node.borrow();
            Self::go(&n.left, v);
            v.iter_mut().for_each(|x| *x += n.val);
            v.push(n.val);
            Self::go(&n.right, v);
        }
    }

    fn change(node: &Option<Rc<RefCell<TreeNode>>>, v: &Vec<i32>, mut idx: usize) -> usize {
        if let Some(node) = node {
            let mut n = node.borrow_mut();
            idx = Self::change(&n.left, v, idx);
            n.val = v[idx];
            idx += 1;
            idx = Self::change(&n.right, v, idx);
        }

        return idx;
    }

    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let mut v = Vec::new();

        Self::go(&root, &mut v);
        Self::change(&root, &v, 0);

        root
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{:?}", Solution::convert_bst(root));
}
