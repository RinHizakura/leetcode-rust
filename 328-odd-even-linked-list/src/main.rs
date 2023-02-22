use utils::list;
use utils::listnode::*;

struct Solution;
impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_dummy_node = ListNode::new(0);
        let mut even_dummy_node = ListNode::new(0);

        let mut cur_odd = &mut odd_dummy_node;
        let mut cur_even = &mut even_dummy_node;

        let mut cnt = 0;
        while let Some(ref mut n) = head {
            let next = n.next.take();

            if cnt & 1 == 1 {
                cur_odd.next = head.take();
                cur_odd = cur_odd.next.as_mut().unwrap();
            } else {
                cur_even.next = head.take();
                cur_even = cur_even.next.as_mut().unwrap();
            }

            head = next;
            cnt += 1;
        }

        cur_even.next = odd_dummy_node.next.take();

        even_dummy_node.next.take()
    }
}

fn main() {
    println!("{:?}", Solution::odd_even_list(Some(list![0, 1, 2, 3, 4])));
}
