use regex::Regex;
use std::rc::Rc;

use super::parser::Parser;
use super::parseresult::ParseResult;
use crate::element::Element;

#[derive(Debug)]
pub struct Grammar {
    entry: Rc<dyn Element>,
    keyword_re: Regex,
}

impl Grammar {
    pub fn new(entry: Rc<dyn Element>, keyword_re: Option<Regex>) -> Grammar {
        let keyword_re = keyword_re.unwrap_or_else(|| Regex::new(r"^\\w+").unwrap());

        Grammar { entry, keyword_re }
    }

    pub fn parse(&self, query: &str) -> ParseResult {
        let parser = Parser::new(query, &self.keyword_re);

        self.entry.parse(&parser);

        let pr = ParseResult::new(false, query, parser.node);

        pr
    }
}
