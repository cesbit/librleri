use super::{Elem, Element, Kind};
use crate::parsing::node::Node;
use crate::parsing::parser::Parser;
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
        Kind::Sequence
    }

    fn parse(&self, _elem: &Elem, _parser: &mut Parser, _parent: &mut Node) -> Option<Node> {
        None
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Sequence {
    pub fn new(elements: Vec<Elem>) -> Elem {
        Rc::new(Self { id: None, elements })
    }

    pub fn with_id(id: i32, elements: Vec<Elem>) -> Elem {
        Rc::new(Self {
            id: Some(id),
            elements,
        })
    }
}

#[macro_export]
macro_rules! sequence {
    ( $id:expr; $( $elements:expr ),* ) => {
        {
            let mut elements = Vec::new();
            $(
                elements.push(std::rc::Rc::clone($elements));
            )*
            crate::element::sequence::Sequence::with_id($id, elements)
        }
    };

    ( $( $elements:expr ),* ) => {
        {
            let mut elements = Vec::new();
            $(
                elements.push(std::rc::Rc::clone($elements));
            )*
            crate::element::sequence::Sequence::new(elements)
        }
    };
}
