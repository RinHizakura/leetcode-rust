use utils::list;
use utils::listnode::ListNode;

struct Solution;
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut list_less = Some(Box::new(ListNode::new(1)));
        let mut list_large_eq = Some(Box::new(ListNode::new(1)));
        let mut cur_less = &mut list_less;
        let mut cur_large_eq = &mut list_large_eq;

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                cur_less.as_deref_mut().unwrap().next = Some(node);
                cur_less = &mut cur_less.as_deref_mut().unwrap().next;
            } else {
                cur_large_eq.as_deref_mut().unwrap().next = Some(node);
                cur_large_eq = &mut cur_large_eq.as_deref_mut().unwrap().next;
            }
        }

        cur_less.as_deref_mut().unwrap().next = list_large_eq.as_deref_mut().unwrap().next.take();

        list_less.as_deref_mut().unwrap().next.take()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::partition(Some(list![1, 4, 3, 2, 5, 2],), 3)
    );
}
