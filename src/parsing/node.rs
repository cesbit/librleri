pub struct Node {
    children: Vec<Node>,
    pub pos: usize,
    pub len: usize,
}

impl Node {
    pub(crate) fn new(pos: usize) -> Node {
        Node {
            pos,
            children: Vec::new(),
            len: 0,
        }
    }
}