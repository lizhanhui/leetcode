struct MyStack {
    q1: std::collections::VecDeque<i32>,
    q2: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self {
            q1: std::collections::VecDeque::new(),
            q2: std::collections::VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.q1.is_empty() {
            self.q1.push_back(x);
            return;
        }

        while let Some(v) = self.q1.pop_front() {
            self.q2.push_back(v);
        }

        self.q1.push_back(x);
        while let Some(v) = self.q2.pop_front() {
            self.q1.push_back(v);
        }
    }

    fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q1.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}
//Your MyStack object will be instantiated and called as such:
//let obj = MyStack::new();
//obj.push(x);
//let ret_2: i32 = obj.pop();
//let ret_3: i32 = obj.top();
//let ret_4: bool = obj.empty();
