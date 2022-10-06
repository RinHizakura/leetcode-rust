use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn recursive(
        left: i32,
        right: i32,
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        i: &mut usize,
        len: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right || *i >= len {
            return None;
        }

        let val = preorder[*i];
        let index = inorder.iter().position(|&r| r == val).unwrap() as i32;
        let mut root = TreeNode::new(val);
        *i += 1;
        root.left = Self::recursive(left, index - 1, preorder, inorder, i, len);
        root.right = Self::recursive(index + 1, right, preorder, inorder, i, len);

        Some(Rc::new(RefCell::new(root)))
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut i = 0;
        let len = preorder.len();

        Self::recursive(0, len as i32, &preorder, &inorder, &mut i, len)
    }
}
fn main() {
    println!("Hello, world!");
}
