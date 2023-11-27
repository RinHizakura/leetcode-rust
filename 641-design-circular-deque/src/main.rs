struct MyCircularDeque {
    inner: Vec<i32>,
    cap: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let cap = k as usize;
        MyCircularDeque {
            inner: vec![0; cap],
            cap: cap,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        /* Insert front like insert last for the corner case */
        if self.is_empty() {
            return self.insert_last(value);
        }

        if self.head > 0 {
            self.head -= 1
        } else {
            self.head = self.cap - 1;
        };
        let idx = self.head;
        self.inner[idx] = value;

        self.count += 1;
        return true;
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let idx = self.tail % self.cap;
        self.inner[idx] = value;
        if self.tail + 1 < self.cap {
            self.tail += 1;
        } else {
            self.tail = 0;
        }

        self.count += 1;
        return true;
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.head + 1 < self.cap {
            self.head += 1;
        } else {
            self.head = 0;
        }
        self.count -= 1;
        return true;
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.tail > 0 {
            self.tail -= 1
        } else {
            self.tail = self.cap - 1;
        };
        self.count -= 1;
        return true;
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let idx = self.head;
        return self.inner[idx];
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let idx = if self.tail > 0 {
            self.tail - 1
        } else {
            self.cap - 1
        };
        return self.inner[idx];
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count >= self.cap
    }
}

fn main() {
    let mut obj = MyCircularDeque::new(5);
    let ret_1: bool = obj.insert_front(1);
    println!("{}", ret_1);
    let ret_2: bool = obj.insert_last(2);
    println!("{}", ret_2);
    let ret_3: bool = obj.delete_front();
    println!("{}", ret_3);
    let ret_4: bool = obj.delete_last();
    println!("{}", ret_4);
    let ret_5: i32 = obj.get_front();
    println!("{}", ret_5);
    let ret_6: i32 = obj.get_rear();
    println!("{}", ret_6);
    let ret_7: bool = obj.is_empty();
    println!("{}", ret_7);
    let ret_8: bool = obj.is_full();
    println!("{}", ret_8);
}
