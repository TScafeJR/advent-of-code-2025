#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkedListNode<T> {
    pub value: T,
    pub next: Option<Box<LinkedListNode<T>>>,
}

impl<T: Eq + Clone> LinkedListNode<T> {
    pub fn new(v: T) -> LinkedListNode<T> {
        LinkedListNode {
            value: v,
            next: None,
        }
    }

    #[allow(dead_code)]
    pub fn has_children(&self) -> bool {
        !self.next.is_some()
    }

    #[allow(dead_code)]
    pub fn get_next(&self) -> Option<&LinkedListNode<T>> {
        self.next.as_ref().map(|node| node.as_ref())
    }

    #[allow(dead_code)]
    pub fn append(&mut self, new_value: T) {
        let mut cursor = self;

        while let Some(ref mut next_node) = cursor.next {
            cursor = next_node;
        }

        cursor.next = Some(Box::new(LinkedListNode::new(new_value)));
    }
}
