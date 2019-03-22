use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::{Elem, Element, Kind};
use crate::parsing::node::Node;
use crate::parsing::parser::Parser;

#[derive(Debug)]
pub struct Forward {
    id: Option<i32>,
    fwd: Option<Elem>,
}

impl Forward {
    pub fn new() -> Elem {
        Rc::new(RefCell::new(Forward {
            id: None,
            fwd: None,
        }))
    }

    pub fn with_id(id: i32) -> Elem {
        Rc::new(RefCell::new(Forward {
            id: Some(id),
            fwd: None,
        }))
    }

    pub fn set_forward(&mut self, elem: &Elem) {
        self.fwd = Some(Rc::clone(elem));
    }
}

impl Element for Forward {
    fn id(&self) -> Option<i32> {
        self.id
    }

    fn free(&mut self) {
        self.fwd = None;
    }

    fn parse(&self, _elem: &Elem, parser: &mut Parser, parent: &mut Node) -> bool {
        match self.fwd {
            Some(ref e) => e.borrow().parse(e, parser, parent),
            None => false
        }
    }

    fn kind(&self) -> Kind {
        Kind::Forward
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.kind())
    }

    fn as_mut_forward(&mut self) -> Option<&mut Forward> {
        Some(self)
    }
}

#[macro_export]
macro_rules! forward {
    ( ) => {
        crate::element::forward::Forward::new()
    };

    ( $id:expr; ) => {
        crate::element::forward::Forward::with_id($id)
    };
}
