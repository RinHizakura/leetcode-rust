use utils::listnode::ListNode;
use utils::list;

struct Solution;
impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new = ListNode::new(std::i32::MIN);
        new.next = head;

        let mut head = None;
        if let Some(ref mut node) = new.next {
            head = node.next.take();
        }

        let mut new = Some(Box::new(new));
        while let Some(mut node) = head {
            head = node.next.take();

            let mut cur = &mut new;

            while let Some(n) = cur {
                if n.next.is_some() && n.next.as_mut().unwrap().val > node.val {
                    node.next = n.next.take();
                    n.next = Some(node);
                    break;
                } else if n.next.is_none() {
                    n.next = Some(node);
                    break;
                }
                cur = &mut n.next;
            }
        }

        new.unwrap().next
    }
}

fn main() {
    println!("{:?}", Solution::insertion_sort_list(Some(list![3,2,1,4])));
}
