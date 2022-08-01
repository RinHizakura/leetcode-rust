use utils::listnode::ListNode;
use utils::list;

struct Solution;
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
       let mut dummy = ListNode::new(1);
       let mut cur = &mut dummy.next;

       let mut first = None;
       while let Some(mut node) = head.take() {
           head = node.next.take();

           match first.take() {
               Some(n) => {
                   node.next = Some(n);
                   *cur = Some(node);
                   cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
               }
               None => first = Some(node),
           }
       }

       *cur = first;
       dummy.next
    }
}

fn main() {
    println!("{:?}", Solution::swap_pairs(Some(list![1, 2, 3])));
}
