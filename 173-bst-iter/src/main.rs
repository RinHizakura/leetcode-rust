use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct BSTIterator {
    q: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let q = Vec::new();
        let mut this = Self { q: q };
        this.update_stack(root);
        this
    }

    fn update_stack(&mut self, mut node: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(n) = node {
            node = n.borrow().left.clone();
            self.q.push(n);
        }
    }

    fn next(&mut self) -> i32 {
        let node = self.q.pop().unwrap();
        let v = node.borrow().val;

        if node.borrow().right.is_some() {
            self.update_stack(node.borrow_mut().right.take());
        }

        v
    }

    fn has_next(&self) -> bool {
        !self.q.is_empty()
    }
}

fn main() {
    let mut obj = BSTIterator::new(Some(Rc::new(RefCell::new(TreeNode::new(2)))));
    let ret_2: bool = obj.has_next();
    let ret_1: i32 = obj.next();
    println!("{} {}", ret_1, ret_2);
}
