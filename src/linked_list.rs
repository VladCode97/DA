use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub data: String,
    pub next: Option<Rc<*mut Node>>,
}

#[derive(Debug)]
pub struct LinkedList {
    pub head: Node,
}