#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn size(&self) -> usize {
        self.top
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(4);
    println!("top {:?}, size {}", s.peek().unwrap(), s.size());
    println!("pop {:?}, size {}", s.pop().unwrap(), s.size());
    println!("is_empty {}, stack {:?}", s.is_empty(), s);
}
