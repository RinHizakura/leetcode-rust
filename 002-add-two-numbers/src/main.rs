// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/* FIXME: Such an ugly macro, we may write this better
 * Reference: https://askeing.github.io/rust-book/macros.html
 */
#[macro_export]
macro_rules! list {
     ($first:expr) => {
         {
             let head = Box::new(ListNode::new($first));
             head
         }
     };
     ($first:expr, $($x:expr), *) => {
        {
            /* Create the first node */
            let mut head = Box::new(ListNode::new($first));
            let mut cur = &mut head;

            $(
            cur.next = Some(Box::new(ListNode::new($x)));
            cur = cur.next.as_mut().unwrap();
            )*

            // surpress warning
            let _ = cur;
            head
        }
    };
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // the first element is guaranteed to exist
    let mut p1 = &l1;
    let mut p2 = &l2;
    let n = p1.as_ref().unwrap().val + p2.as_ref().unwrap().val;

    let mut head = Box::new(ListNode::new(n % 10));
    let mut cur = &mut head;
    let mut carry = n / 10;

    p1 = &p1.as_ref().unwrap().next;
    p2 = &p2.as_ref().unwrap().next;

    while let (Some(n1), Some(n2)) = (p1, p2) {
        let n = n1.val + n2.val + carry;
        carry = n / 10;
        cur.next = Some(Box::new(ListNode::new(n % 10)));
        cur = cur.next.as_mut().unwrap();

        p1 = &n1.next;
        p2 = &n2.next;
    }

    while let Some(n1) = p1 {
        let n = n1.val + carry;
        carry = n / 10;
        cur.next = Some(Box::new(ListNode::new(n % 10)));
        cur = cur.next.as_mut().unwrap();
        p1 = &n1.next;
    }

    while let Some(n2) = p2 {
        let n = n2.val + carry;
        carry = n / 10;
        cur.next = Some(Box::new(ListNode::new(n % 10)));
        cur = cur.next.as_mut().unwrap();
        p2 = &n2.next;
    }

    if carry != 0 {
        cur.next = Some(Box::new(ListNode::new(carry)));
    }

    return Some(head);
}

fn main() {
    let l1 = Some(list![9, 9, 9]);
    let l2 = Some(list![9]);

    println!("{:?}", add_two_numbers(l1, l2));
}
