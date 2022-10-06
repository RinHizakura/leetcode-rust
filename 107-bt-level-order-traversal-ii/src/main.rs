use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn submit(
        a: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        b: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        let mut vec = vec![];
        let mut flag = 0;
        while let Some(node) = a.pop() {
            if let Some(n) = node {
                flag = 1;
                let val = n.borrow().val;
                vec.push(val);
                b.insert(0, n.borrow().left.clone());
                b.insert(0, n.borrow().right.clone());
            }
        }
        if flag != 0 {
            ans.insert(0, vec);
        }
    }

    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let mut container_0 = vec![];
        let mut container_1 = vec![];
        container_0.push(root.clone());

        loop {
            let mut b = 0;
            if !container_0.is_empty() {
                Solution::submit(&mut container_0, &mut container_1, &mut ans);
                b = 1;
            }

            if !container_1.is_empty() {
                Solution::submit(&mut container_1, &mut container_0, &mut ans);
                b = 1;
            }

            if b == 0 {
                break;
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
