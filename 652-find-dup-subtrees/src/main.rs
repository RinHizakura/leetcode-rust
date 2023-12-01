use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    fn go(
        node: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<String, i32>,
        ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        if let Some(n) = node {
            let val = n.borrow().val;
            let left = Self::go(&n.borrow().left, map, ans);
            let right = Self::go(&n.borrow().right, map, ans);
            let s = format!("{},{},{}", val, left, right);

            let value = map.entry(s.to_owned()).or_insert(0);
            *value += 1;

            if *value == 2 {
                ans.push(node.clone());
            }

            s
        } else {
            // represent NULL
            "N".to_string()
        }
    }

    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut map = HashMap::new();
        let mut ans = Vec::new();
        Self::go(&root, &mut map, &mut ans);

        ans
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{:?}", Solution::find_duplicate_subtrees(root));
}
