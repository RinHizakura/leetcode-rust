struct MinStack {
    /* the first element is the value which is pushed to stack
     * the second element is the minimum if such entry is the top of stack */
    stack: Vec<(i32, i32)>,
    size: usize,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, val: i32) {
        if self.size == 0 {
            self.stack.push((val, val));
        } else {
            let m = self.get_min();
            self.stack.push((val, m.min(val)));
        }
        self.size += 1;
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.size -= 1;
    }

    fn top(&self) -> i32 {
        self.stack[self.size - 1].0
    }

    fn get_min(&self) -> i32 {
        self.stack[self.size - 1].1
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(1);
    obj.push(-1);
    let ret_1: i32 = obj.top();
    let ret_2: i32 = obj.get_min();
    obj.pop();
    let ret_3: i32 = obj.get_min();
    println!("{} {} {}", ret_1, ret_2, ret_3);
}
