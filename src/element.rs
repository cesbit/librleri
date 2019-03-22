use crate::parsing::node::Node;
use crate::parsing::parser::Parser;
use std::cell::RefCell;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub mod forward;
pub mod keyword;
pub mod sequence;

pub type Elem = Rc<RefCell<dyn Element>>;

#[derive(Debug)]
pub enum Kind {
    Forward,
    Keyword,
    Sequence,
}

pub trait Element {
    fn parse(&self, elem: &Elem, parser: &mut Parser, parent: &mut Node) -> bool;
    fn free(&mut self) {}
    fn id(&self) -> Option<i32>;
    fn kind(&self) -> Kind;
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn as_mut_forward(&mut self) -> Option<&mut forward::Forward> {
        None
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
