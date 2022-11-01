use utils::list;
use utils::listnode::ListNode;

struct Solution;
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut len = 0;
        let mut cur = &*head;
        while let Some(n) = cur {
            cur = &n.next;
            len += 1;
        }

        let mut i = 0;
        let mut cur = &mut *head;
        let mut mid = None;
        while let Some(n) = cur {
            cur = &mut n.next;
            i += 1;
            if i * 2 >= len {
                mid = cur.take();
                break;
            }
        }

        let mut first = None;
        while let Some(mut node) = mid.take() {
            mid = node.next.take();
            match first {
                Some(n) => {
                    node.next = Some(n);
                    first = Some(node)
                }
                None => first = Some(node),
            }
        }

        let mut tail = None;
        if let Some(node) = head {
            tail = node.next.take();
        }

        let mut cur = &mut *head;
        while let Some(mut n1) = first.take() {
            first = n1.next.take();
            if let Some(mut n2) = tail.take() {
                tail = n2.next.take();
                n1.next = Some(n2);
            }

            if let Some(n) = cur {
                n.next = Some(n1);
                cur = &mut n.next.as_mut().unwrap().next;
            }
        }
    }
}

fn main() {
    let mut l = Some(list![1, 2, 3, 4]);
    Solution::reorder_list(&mut l);
    println!("{:?}", l);
}
