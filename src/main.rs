use std::rc::Rc;
use crate::linked_list::{Node, LinkedList};

mod linked_list;

fn main() {
    let mut linked_list = LinkedList {
        head: Node {
            data: String::from("Luis"),
            next: Option::None,
        }
    };
    let mut node_two = Node {
        data: String::from("Natalia"),
        next: Option::None,
    };
    let mut node_three = Node {
        data: String::from("Palomo"),
        next: Option::None,
    };
    //Linked list
    linked_list.head.next = Option::Some(Rc::new(&mut node_two));
    node_two.next = Option::Some(Rc::new(&mut node_three));
    node_three.next = Option::None;
    dbg!(linked_list);
    dbg!(node_two);
    dbg!(node_three);
}
