struct MyCircularQueue {
    inner: Vec<i32>,
    cap: usize,
    head: usize,
    tail: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let cap = k as usize;
        MyCircularQueue {
            inner: vec![0; cap],
            cap: cap,
            head: 0,
            tail: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let idx = self.tail % self.cap;
        self.inner[idx] = value;
        self.tail += 1;
        return true;
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.head += 1;
        return true;
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let idx = self.head % self.cap;
        return self.inner[idx];
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let idx = (self.tail - 1) % self.cap;
        return self.inner[idx];
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        if self.tail >= self.head {
            self.tail - self.head >= self.cap
        } else {
            self.tail + (usize::MAX - self.head) >= self.cap
        }
    }
}

fn main() {
    let mut obj = MyCircularQueue::new(3);
    let ret_1: bool = obj.en_queue(1);
    let ret_2: bool = obj.de_queue();
    let ret_3: i32 = obj.front();
    let ret_4: i32 = obj.rear();
    let ret_5: bool = obj.is_empty();
    let ret_6: bool = obj.is_full();
    println!("{ret_1} {ret_2} {ret_3} {ret_4} {ret_5} {ret_6}");
}
