use crate::util::nodes;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct LinkedList<T> {
    pub head: Option<Box<nodes::LinkedListNode<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    #[allow(dead_code)]
    pub fn add_head(&mut self, mut n: nodes::LinkedListNode<T>) {
        n.next = self.head.take();

        self.head = Some(Box::new(n));
    }

    #[allow(dead_code)]
    pub fn add_node(&mut self, n: nodes::LinkedListNode<T>) {
        if self.head.is_none() {
            self.head = Some(Box::new(n));
            return;
        }

        let mut curr = self.head.as_mut().unwrap();

        while let Some(ref mut next_node) = curr.next {
            curr = next_node;
        }

        curr.next = Some(Box::new(n));
    }
}

impl<T: Eq + Debug> LinkedList<T> {
    #[allow(dead_code)]
    pub fn print_list(&self) {
        println!("{:?}", self.head);
    }
}
