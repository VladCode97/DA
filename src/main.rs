use crate::queue::Queue;

mod stack;
mod queue;

fn main() {
    let mut queue = Queue {
        queue: vec![],
        top: 0,
        rear: 0,
    };
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.print();
    queue.dequeue();
    dbg!(&queue.peek());
    queue.dequeue();
    queue.print();
}
