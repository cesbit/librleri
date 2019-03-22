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
pub enum Kind<'k> {
    Forward(&'k forward::Forward),
    Keyword(&'k keyword::Keyword),
    Sequence(&'k sequence::Sequence),
}

pub trait Element {
    fn parse(&self, this: &Elem, parser: &mut Parser, parent: &mut Node) -> bool;
    fn free(&mut self) {}
    fn id(&self) -> Option<i32>;
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn kind(&self) -> Kind;
    fn kind_mut(&mut self) -> Kind;
}

impl Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
