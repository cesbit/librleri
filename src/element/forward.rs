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

    pub fn set_forward(&mut self, elem: &Elem) {
        self.fwd = Some(Rc::clone(elem));
        self.id = elem.borrow().id();
    }
}

impl Element for Forward {
    fn id(&self) -> Option<i32> {
        self.id
    }

    fn unknot(&mut self) {
        self.fwd = None;
    }

    fn parse(&self, _elem: &Elem, parser: &mut Parser, parent: &mut Node) -> bool {
        match self.fwd {
            Some(ref e) => e.borrow().parse(e, parser, parent),
            None => false
        }
    }

    fn kind(&self) -> Kind {
        Kind::Forward(self)
    }

    fn kind_mut(&mut self) -> Kind {
        Kind::ForwardMut(self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Fwd>")
    }
}

#[macro_export]
macro_rules! forward {
    ( ) => {
        crate::element::forward::Forward::new()
    };
}


#[macro_export]
macro_rules! forward_set {
    (  $fwd:expr, $elem:expr) => {
        // The extra `{` block is required so the `ref_mut_fwd` borrow will
        // be returned when the block is closed.
        {
            let mut ref_mut_fwd = $fwd.borrow_mut();
            match ref_mut_fwd.kind_mut() {
                crate::element::Kind::ForwardMut(ref mut fwd) => fwd.set_forward($elem),
                _ => panic!("macro `forward_set` expects a `Forward` element as first argument"),
            };
        }
    };
}
