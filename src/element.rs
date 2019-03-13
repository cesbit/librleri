use std::fmt::{self, Debug};

use crate::parsing::node::Node;
use crate::parsing::parser::Parser;

pub mod keyword;
pub mod sequence;

pub enum Kind {
    Keyword,
    Sequence,
}

pub trait Element {
    fn parse(&self, parser: &Parser) -> Result<Node, &'static str>;
    fn id(&self) -> Option<i32>;
    fn kind(&self) -> Kind;
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
