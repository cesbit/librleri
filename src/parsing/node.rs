use crate::element::Elem;

pub struct Node {
    pub pos: usize,
    pub len: usize,
    pub elem: Elem,
    pub children: Vec<Node>
}

impl Node {
    pub(crate) fn new(pos: usize, len: usize, elem: Elem) -> Node {
        Node {
            pos,
            len,
            elem,
            children: Vec::new(),
        }
    }
}
