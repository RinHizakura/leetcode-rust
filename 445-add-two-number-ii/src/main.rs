use utils::list;
use utils::listnode::ListNode;

struct Solution;
impl Solution {
    fn solver(
        mut cur1: &Option<Box<ListNode>>,
        mut cur2: &Option<Box<ListNode>>,
        len1: usize,
        len2: usize,
        idx: usize,
    ) -> (i32, Option<Box<ListNode>>) {
        if idx == 0 {
            return (0, None);
        }

        let mut val1 = 0;
        if idx <= len1 {
            val1 = cur1.as_ref().unwrap().val;
            cur1 = &cur1.as_ref().unwrap().next;
        }

        let mut val2 = 0;
        if idx <= len2 {
            val2 = cur2.as_ref().unwrap().val;
            cur2 = &cur2.as_ref().unwrap().next;
        }

        let (carry, node) = Solution::solver(cur1, cur2, len1, len2, idx - 1);
        let sum = val1 + val2 + carry;
        let mut new_node = ListNode::new(sum % 10);
        new_node.next = node;

        (sum / 10, Some(Box::new(new_node)))
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut len1 = 0;
        let mut len2 = 0;

        let mut cur1 = &l1;
        let mut cur2 = &l2;

        let mut update = true;
        while update {
            update = false;

            if cur1.is_some() {
                len1 += 1;
                cur1 = &cur1.as_ref().unwrap().next;
                update = true;
            }

            if cur2.is_some() {
                len2 += 1;
                cur2 = &cur2.as_ref().unwrap().next;
                update = true;
            }
        }

        let (carry, node) = Solution::solver(&l1, &l2, len1, len2, len1.max(len2));
        if carry != 0 {
            let mut new_node = ListNode::new(carry);
            new_node.next = node;
            return Some(Box::new(new_node));
        } else {
            return node;
        }
    }
}

fn main() {
    let l1 = Some(list![9, 9, 9]);
    let l2 = Some(list![9, 9, 9]);
    println!("{:?}", Solution::add_two_numbers(l1, l2));
}
