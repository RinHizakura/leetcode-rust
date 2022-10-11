use std::cell::RefCell;
use std::rc::Rc;
use utils::list;
use utils::listnode::ListNode;
use utils::treenode::TreeNode;

struct Solution;
impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // find the middle node
        let mut fast = &head;
        let mut c = 0;
        while let Some(fast_node) = fast {
            fast = &fast_node.next;
            if let Some(fast_node) = fast {
                fast = &fast_node.next;
                c += 1;
            }
        }

        let mut cur = &mut head;
        for _ in 0..c {
            match cur {
                Some(n) => {
                    cur = &mut n.next;
                }
                None => {
                    assert!(false);
                }
            }
        }

        let root;
        if !cur.is_none() {
            let cur = cur.take();
            let mut r = TreeNode::new(cur.as_ref().unwrap().val);
            r.right = Solution::sorted_list_to_bst(cur.unwrap().next.take());
            r.left = Solution::sorted_list_to_bst(head);
            root = Some(Rc::new(RefCell::new(r)));
        } else {
            root = None;
        }

        root
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sorted_list_to_bst(Some(list![-10, -3, 0, 5, 9]))
    );
}
