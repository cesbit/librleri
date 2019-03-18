use regex::Regex;
use super::parser::Parser;
use super::parseresult::ParseResult;
use super::node::Node;
use crate::element::Elem;

#[derive(Debug)]
pub struct Grammar {
    entry: Elem,
    keyword_re: Regex,
}

impl Grammar {
    pub fn new(entry: Elem, keyword_re: Option<Regex>) -> Grammar {
        let keyword_re = keyword_re.unwrap_or_else(|| Regex::new(r"^\\w+").unwrap());

        Grammar { entry, keyword_re }
    }

    pub fn parse(&self, query: &str) -> ParseResult {
        let mut parser = Parser::new(query, &self.keyword_re);

        let mut node = Node::new(0, 0, std::rc::Rc::clone(&self.entry));

        parser.walk(&mut node, &self.entry);

        let pr = ParseResult::new(false, query, node);

        pr
    }
}
