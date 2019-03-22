use regex::Regex;
use std::collections::HashMap;
use super::node::Node;
use crate::element::Elem;

pub struct Parser<'a> {
    query: &'a str,
    keyword_re: &'a Regex,
    kw_cache: HashMap<usize, Option<&'a str>>,
    // pub node: Node,
}

impl<'a> Parser<'a> {

    pub fn new(query: &'a str, keyword_re: &'a Regex) -> Parser<'a> {
        Parser {
            query,
            keyword_re,
            kw_cache: HashMap::new(),
            // node: Node::new(0, entry),
        }
    }

    pub fn kw_match(&mut self, pos: usize) -> &Option<&'a str> {
        let query = &self.query;
        let keyword_re = &self.keyword_re;

        self.kw_cache
            .entry(pos)
            .or_insert_with(|| match keyword_re.find(&query[pos..]) {
                Some(x) => Some(&query[pos..x.end()]),
                None => None,
            })
    }

    pub fn walk(&mut self, parent: &mut Node, elem: &Elem) -> bool {
        let mut char_indices = self.query.char_indices();
        loop {
            match char_indices.next() {
                Some((i, c)) => {
                    if !c.is_whitespace() {
                        parent.len = i;
                        break;
                    }
                }
                None => break,
            }
        }

        elem.borrow().parse(elem, self, parent)
    }
}