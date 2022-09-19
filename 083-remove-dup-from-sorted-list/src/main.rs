use utils::list;
use utils::listnode::ListNode;

struct Solution;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(1)));
        let mut cur = &mut dummy;
        let mut prev_val = 101;

        while let Some(ref mut node) = head {
            if node.val == prev_val {
                head = node.next.take();
                continue;
            }

            prev_val = node.val;
            cur.as_deref_mut().unwrap().next = head.take();
            cur = &mut cur.as_deref_mut().unwrap().next;
            head = cur.as_deref_mut().unwrap().next.take();
        }

        dummy.as_deref_mut().unwrap().next.take()
    }
}

fn main() {
    let v = Some(list![1, 2, 3]);
    println!("{:?}", Solution::delete_duplicates(v));
}
