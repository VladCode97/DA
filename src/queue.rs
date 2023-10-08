#[derive(Debug)]
pub struct Queue {
    pub queue: Vec<u32>,
    pub top: usize,
    pub rear: usize,
}

impl Queue {
    //  0       0 , 1      0 , 1, 2
    // [3] --- [3, 2] --- [3, 2, 5]
    pub fn enqueue(&mut self, element: u32) -> () {
        self.queue.insert(self.top, element);
        self.top += 1;
    }
    //  0       0 , 1      1, 2
    // [3] --- [3, 2] --- [3, 2, 5]
    pub fn dequeue(&mut self) -> () {
        self.queue.remove(self.rear);
    }

    pub fn print(&self) {
        dbg!(&self.queue);
    }

    pub fn peek(&self) -> u32 {
        self.queue[self.rear]
    }
}

