use utils::list;
use utils::listnode::*;

struct Solution;
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head == None {
            return head;
        }

        let mut p1 = &head;
        let mut len = 0;
        for _ in 0..k {
            match p1 {
                Some(n1) => {
                    p1 = &n1.next;
                    len += 1
                }
                None => break,
            }
        }

        if *p1 == None {
            return Solution::rotate_right(head, k % len);
        }

        let mut cnt = 0;
        while let Some(n1) = p1 {
            p1 = &n1.next;
            cnt += 1;
        }

        let mut p2 = &mut head;
        for _ in 0..cnt {
            match p2 {
                Some(n2) => p2 = &mut n2.next,
                None => panic!(),
            }
        }

        let mut new_head = p2.take();
        let mut p3 = &mut new_head;
        for _ in 0..k {
            match p3 {
                Some(n3) => p3 = &mut n3.next,
                None => break,
            }
        }

        *p3 = head;

        new_head
    }
}

fn main() {
    println!("{:?}", Solution::rotate_right(Some(list![0, 1, 2]), 0));
    println!("{:?}", Solution::rotate_right(Some(list![0, 1, 2]), 1));
    println!("{:?}", Solution::rotate_right(Some(list![0, 1, 2]), 2));
}
