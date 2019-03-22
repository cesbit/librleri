use super::node::Node;
use super::parser::Parser;
use super::parseresult::ParseResult;
use crate::element::Elem;
use regex::Regex;
use std::rc::Rc;

#[derive(Debug)]
pub struct Grammar {
    entry: Elem,
    keyword_re: Regex,
}

impl Grammar {
    pub fn new(entry: &Elem, keyword_re: Option<Regex>) -> Grammar {
        let keyword_re = keyword_re.unwrap_or_else(|| Regex::new(r"^\w+").unwrap());

        Grammar {
            entry: Rc::clone(entry),
            keyword_re,
        }
    }

    pub fn parse(&self, query: &str) -> ParseResult {
        let mut parser = Parser::new(query, &self.keyword_re);

        let mut node = Node::new(0, 0, std::rc::Rc::clone(&self.entry));

        let is_valid = parser.walk(&mut node, &self.entry);

        let pr = ParseResult::new(is_valid, query, node);

        pr
    }
}

impl Drop for Grammar {
    fn drop(&mut self) {
        self.entry.borrow_mut().free();
    }
}