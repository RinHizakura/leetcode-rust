// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
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
