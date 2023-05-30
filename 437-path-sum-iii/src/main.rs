use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn traverse(
        cur: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        deque: &mut VecDeque<usize>,
        ans: &mut i32,
    ) {
        if let Some(node) = cur {
            let borrowed = node.borrow();
            let val = borrowed.val;

            if val == target {
                *ans += 1;
            }

            for idx in 0..deque.len() {
                deque[idx] += val as usize;
                if deque[idx] == target as usize {
                    *ans += 1;
                }
            }
            deque.push_back(val as usize);
            Solution::traverse(&borrowed.left, target, deque, ans);
            Solution::traverse(&borrowed.right, target, deque, ans);
            deque.pop_back();
            for idx in 0..deque.len() {
                deque[idx] -= val as usize;
            }
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut ans = 0;
        let mut deque: VecDeque<usize> = VecDeque::new();

        Solution::traverse(&root, target_sum, &mut deque, &mut ans);
        ans
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    println!("{}", Solution::path_sum(root, 3));
}
