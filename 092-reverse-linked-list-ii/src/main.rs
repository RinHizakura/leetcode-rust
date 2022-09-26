use utils::list;
use utils::listnode::ListNode;

struct Solution;
impl Solution {
    fn reverse_list(
        mut head: Option<Box<ListNode>>,
        mut tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev = head;
        let mut cur = match prev.as_deref_mut() {
            Some(node) => node.next.take(),
            None => return tail,
        };
        prev.as_deref_mut().unwrap().next = tail.take();

        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }

    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(1)));
        dummy.as_deref_mut().unwrap().next = head;
        let mut cur_head = &mut dummy;
        for _ in 1..left {
            cur_head = &mut cur_head.as_deref_mut().unwrap().next;
        }

        let mut mid = cur_head.as_deref_mut().unwrap().next.take();
        let mut cur_mid = &mut mid;
        let total = right - left;
        for _ in 0..total {
            cur_mid = &mut cur_mid.as_deref_mut().unwrap().next;
        }

        let mut tail = match cur_mid.as_deref_mut() {
            Some(node) => node.next.take(),
            None => None,
        };
        let mut l = Solution::reverse_list(mid, tail.take());
        cur_head.as_deref_mut().unwrap().next = l.take();
        dummy.as_deref_mut().unwrap().next.take()
    }
}

fn main() {
    println!("{:?}", Solution::reverse_between(Some(list![5]), 1, 1));
    println!("{:?}", Solution::reverse_between(Some(list![3, 5]), 1, 1));
    println!("{:?}", Solution::reverse_between(Some(list![3, 5]), 1, 2));
}
