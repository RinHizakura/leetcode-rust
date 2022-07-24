use utils::list;
use utils::listnode::*;

/* Tips: For list with length "len", removing the nth node from end means
 * to change the next of "len - nth" node, which means that we can traverse
 * "len - nth" times to find that node. */

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        /* create a mutable dummy node which takes the ownership of head */
        let mut dummy = Some(Box::new(ListNode::new(1)));
        dummy.as_deref_mut().unwrap().next = head;

        let mut p1 = dummy.as_deref();

        for _ in 0..n + 1 {
            match p1 {
                Some(n1) => p1 = n1.next.as_deref(),
                None => return dummy.unwrap().next,
            };
        }

        let mut cnt = 0;
        while let Some(n1) = p1 {
            p1 = n1.next.as_deref();
            cnt += 1;
        }

        let mut p2 = dummy.as_deref_mut();
        for _ in 0..cnt {
            p2 = p2.unwrap().next.as_deref_mut();
        }
        let next = p2
            .as_deref_mut()
            .unwrap()
            .next
            .as_deref_mut()
            .unwrap()
            .next
            .take();
        p2.unwrap().next = next;

        dummy.unwrap().next
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::remove_nth_from_end(Some(list![1, 2, 3, 4, 5]), 2)
    );
}
