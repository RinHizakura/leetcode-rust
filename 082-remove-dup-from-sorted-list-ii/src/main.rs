use utils::list;
use utils::listnode::ListNode;

struct Solution;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(1)));
        let mut cur = &mut dummy;
        let mut prev_val = 101;

        while let Some(ref mut node) = head {
            let cur_val = node.val;
            let next = &mut node.next;

            let mut cond = true;
            if next.is_some() {
                cond = cur_val != next.as_deref_mut().unwrap().val;
            }

            if cur_val != prev_val && cond {
                cur.as_deref_mut().unwrap().next = head.take();
                cur = &mut cur.as_deref_mut().unwrap().next;
                head = cur.as_deref_mut().unwrap().next.take();
            } else {
                head = next.take();
            }

            prev_val = cur_val;
        }

        dummy.as_deref_mut().unwrap().next.take()
    }
}

fn main() {
    let l = Some(list![1, 2, 3, 3, 4, 4, 5]);
    println!("{:?}", Solution::delete_duplicates(l));
}
