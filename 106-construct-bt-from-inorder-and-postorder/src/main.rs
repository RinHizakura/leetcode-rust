use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn recursive(
        left: i32,
        right: i32,
        postorder: &Vec<i32>,
        inorder: &Vec<i32>,
        i: &mut i32,
        len: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right || *i < 0 {
            return None;
        }

        let val = postorder[*i as usize];
        let index = inorder.iter().position(|&r| r == val).unwrap() as i32;
        let mut root = TreeNode::new(val);
        *i -= 1;
        root.right = Self::recursive(index + 1, right, postorder, inorder, i, len);
        root.left = Self::recursive(left, index - 1, postorder, inorder, i, len);

        Some(Rc::new(RefCell::new(root)))
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = postorder.len();
        let mut i = len as i32 - 1;

        Self::recursive(0, i, &postorder, &inorder, &mut i, len)
    }
}
fn main() {
    println!("Hello, world!");
}
