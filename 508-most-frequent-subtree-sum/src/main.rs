use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn go(node: &Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, usize>) -> i32 {
        if let Some(n) = node {
            let val = n.borrow().val;
            let left = Self::go(&n.borrow().left, map);
            let right = Self::go(&n.borrow().right, map);

            let mut sum = val;

            if left != i32::MIN {
                sum += left;
            }

            if right != i32::MIN {
                sum += right;
            }

            if let Some(e) = map.get_mut(&sum) {
                *e += 1;
            } else {
                map.insert(sum, 1);
            }
            return sum;
        }

        return i32::MIN;
    }

    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();

        Self::go(&root, &mut map);

        let mut ans = vec![];
        let mut freq_max = 0;
        for (&val, &freq) in &map {
            if freq > freq_max {
                ans.clear();
                ans.push(val);
                freq_max = freq;
            } else if freq == freq_max {
                ans.push(val);
            }
        }
        ans
    }
}

fn main() {
    let left = TreeNode::new(2);
    let right = TreeNode::new(-5);

    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(left))),
        right: Some(Rc::new(RefCell::new(right))),
    };
    let root = Some(Rc::new(RefCell::new(root)));
    println!("{:?}", Solution::find_frequent_tree_sum(root));
}
