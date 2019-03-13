pub struct Node {
    pub pos: usize,
    pub len: usize,
    children: Vec<Node>,
}

impl Node {
    pub(crate) fn new(pos: usize) -> Node {
        Node {
            pos,
            len: 0,
            children: Vec::new(),
        }
    }
}
