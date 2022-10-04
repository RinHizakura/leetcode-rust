use std::cell::RefCell;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn submit(
        a: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        b: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ans: &mut Vec<Vec<i32>>,
        dir: usize,
    ) {
        let mut vec = vec![];
        let mut flag = 0;
        while let Some(node) = a.pop() {
            if let Some(n) = node {
                flag = 1;
                let val = n.borrow().val;
                vec.push(val);
                if dir == 0 {
                    b.push(n.borrow().left.clone());
                    b.push(n.borrow().right.clone());
                } else {
                    b.push(n.borrow().right.clone());
                    b.push(n.borrow().left.clone());
                }
            }
        }
        if flag != 0 {
            ans.push(vec);
        }
    }

    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let mut dir = 0;
        let mut container_0 = vec![];
        let mut container_1 = vec![];
        container_0.push(root.clone());

        while !container_0.is_empty() || !container_1.is_empty() {
            if dir == 0 {
                Solution::submit(&mut container_0, &mut container_1, &mut ans, dir);
            } else {
                Solution::submit(&mut container_1, &mut container_0, &mut ans, dir);
            }
            dir ^= 1;
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
