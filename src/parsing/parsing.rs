pub struct ParseResult {
    query: String,
    is_valid: bool,
    tree: Node,
}


impl ParseResult {
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

