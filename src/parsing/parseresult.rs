use super::node::Node;

pub struct ParseResult {
    is_valid: bool,
    query: String,
    tree: Node,
}

impl ParseResult {
    pub(crate) fn new(is_valid: bool, query: &str, tree: Node) -> ParseResult {
        ParseResult {
            is_valid,
            query: String::from(query),
            tree,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}
