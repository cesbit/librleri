use super::{Elem, Element, Kind};
use crate::parsing::node::Node;
use crate::parsing::parser::Parser;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Sequence {
    id: Option<i32>,
    elements: Vec<Elem>,
}

impl Element for Sequence {
    fn id(&self) -> Option<i32> {
        self.id
    }

    fn kind(&self) -> Kind {
        Kind::Sequence(self)
    }

    fn kind_mut(&mut self) -> Kind {
        Kind::Sequence(self)
    }
    fn parse(&self, this: &Elem, parser: &mut Parser, parent: &mut Node) -> bool {
        let mut node = Node::new(parent.pos, 0, Rc::clone(this));

        for elem in &self.elements {
            if !parser.walk(&mut node, elem) {
                return false;
            }
        }

        parent.len += node.len;
        parent.children.push(node);
        true
    }

    fn free(&mut self) {
        for elem in &self.elements {
            elem.borrow_mut().free();
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Sequence {
    pub fn new(elements: Vec<Elem>) -> Elem {
        Rc::new(RefCell::new(Self { id: None, elements }))
    }

    pub fn with_id(id: i32, elements: Vec<Elem>) -> Elem {
        Rc::new(RefCell::new(Self {
            id: Some(id),
            elements,
        }))
    }
}

#[macro_export]
macro_rules! sequence {
    ( $( $elements:expr ),* ) => {
        {
            let mut elements = Vec::new();
            $(
                elements.push(std::rc::Rc::clone($elements));
            )*
            crate::element::sequence::Sequence::new(elements)
        }
    };

    ( id=$id:expr; $( $elements:expr ),* ) => {
        {
            let mut elements = Vec::new();
            $(
                elements.push(std::rc::Rc::clone($elements));
            )*
            crate::element::sequence::Sequence::with_id($id, elements)
        }
    };
}
