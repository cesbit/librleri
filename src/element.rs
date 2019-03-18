use std::fmt::{self, Debug};

use crate::parsing::node::Node;
use crate::parsing::parser::Parser;

pub mod keyword;
pub mod sequence;

pub type Elem = std::rc::Rc<dyn Element>;

pub enum Kind {
    Keyword,
    Sequence,
}

pub trait Element {
    fn parse(&self, elem: &Elem, parser: &mut Parser, parent: &mut Node) -> Option<Node>;
    fn id(&self) -> Option<i32>;
    fn kind(&self) -> Kind;
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
