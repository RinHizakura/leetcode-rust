use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct ListNode<T> {
    val: T,
    prev: Option<Rc<RefCell<ListNode<T>>>>,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        ListNode {
            val: val,
            prev: None,
            next: None,
        }
    }
}

struct DoublyLinkedList<T> {
    dummy: Rc<RefCell<ListNode<T>>>,
}

impl<T: std::default::Default> DoublyLinkedList<T> {
    pub fn new() -> Self {
        let dummy = Rc::new(RefCell::new(ListNode::new(Default::default())));
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        Self { dummy }
    }

    pub fn push_front(&self, node: Rc<RefCell<ListNode<T>>>) {
        let next = self.dummy.borrow_mut().next.take().unwrap();
        self.dummy.borrow_mut().next = Some(node.clone());
        next.borrow_mut().prev = Some(node.clone());
        node.borrow_mut().prev = Some(self.dummy.clone());
        node.borrow_mut().next = Some(next);
    }

    pub fn pop_back(&self) -> Rc<RefCell<ListNode<T>>> {
        let prev = self.dummy.borrow_mut().prev.take().unwrap();
        let p_prev = prev.borrow().prev.clone().unwrap();

        p_prev.borrow_mut().next = Some(self.dummy.clone());
        self.dummy.borrow_mut().prev = Some(p_prev);
        prev
    }

    pub fn remove(node: Rc<RefCell<ListNode<T>>>) {
        let prev = node.borrow().prev.clone().unwrap();
        let next = node.borrow().next.clone().unwrap();

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }
}

struct LRUCache {
    cap: i32,
    size: i32,
    list: DoublyLinkedList<(i32, i32)>,
    map: HashMap<i32, Rc<RefCell<ListNode<(i32, i32)>>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cap: capacity,
            size: 0,
            list: DoublyLinkedList::new(),
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let node = &*self.map.get(&key).unwrap();
            let value = node.borrow().val.1;

            DoublyLinkedList::remove(node.clone());
            self.list.push_front(node.clone());
            return value;
        }

        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        let new_node = Rc::new(RefCell::new(ListNode::new((key, value))));
        if self.map.contains_key(&key) {
            let node = &*self.map.get(&key).unwrap();

            DoublyLinkedList::remove(node.clone());
        } else {
            if self.size == self.cap {
                let evict = self.list.pop_back();
                self.map.remove(&evict.borrow().val.0);
                self.size -= 1;
            }

            self.size += 1;
        }
        self.list.push_front(new_node.clone());
        self.map.insert(key, new_node);
    }
}

fn main() {
    println!("Hello, world!");
}
