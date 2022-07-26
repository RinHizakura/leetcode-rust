use utils::list;
use utils::listnode::*;

/* FIXME: unwrap, unwrap...... too dirty! */
struct Solution;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(1));
        let mut cur = &mut dummy;

        let mut l1 = Some(Box::new(ListNode::new(1)));
        l1.as_deref_mut().unwrap().next = list1;
        let mut l2 = Some(Box::new(ListNode::new(1)));
        l2.as_deref_mut().unwrap().next = list2;

        let mut n1 = l1.as_deref_mut().unwrap().next.take();
        let mut n2 = l2.as_deref_mut().unwrap().next.take();

        loop {
            if n1.is_none() | n2.is_none() {
                break;
            }

            if n1.as_deref().unwrap().val < n2.as_deref().unwrap().val {
                cur.next = n1.take();
                n1 = cur.next.as_deref_mut().unwrap().next.take();
            } else {
                cur.next = n2.take();
                n2 = cur.next.as_deref_mut().unwrap().next.take();
            }
            cur = cur.next.as_mut().unwrap();
        }

        while n1.is_some() {
            cur.next = n1.take();
            n1 = cur.next.as_deref_mut().unwrap().next.take();
            cur = cur.next.as_mut().unwrap();
        }

        while n2.is_some() {
            cur.next = n2.take();
            n2 = cur.next.as_deref_mut().unwrap().next.take();
            cur = cur.next.as_mut().unwrap();
        }

        dummy.next
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::merge_two_lists(Some(list![-9, 3]), Some(list![5, 7]))
    );
}
