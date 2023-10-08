#[derive(Debug)]
pub struct Stack {
    pub stack: Vec<u32>,
    pub top: usize,
}

impl Stack {
    pub fn push(&mut self, element: u32) {
        self.stack.insert(self.top, element);
        self.top += 1;
    }

    pub fn pop(&mut self) {
        self.stack.remove(self.top - 1);
        self.top -= 1;
    }

    pub fn print(&self) {
        dbg!(&self.stack);
    }

    pub fn peek(&self) -> u32 {
        self.stack[self.top - 1]
    }
}