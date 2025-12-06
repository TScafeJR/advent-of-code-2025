#[allow(dead_code)]
pub struct Node<T> {
    pub value: T,
    pub children: HashMap<Node<T>>,
}
