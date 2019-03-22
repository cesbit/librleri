use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::{Elem, Element, Kind};
use crate::parsing::node::Node;
use crate::parsing::parser::Parser;


#[derive(Debug)]
pub struct Keyword {
    id: Option<i32>,
    keyword: String,
    ignore_case: bool,
}

/// A `Keyword` element.
///
/// Keywords must match a grammar.
///
/// # Examples
///
/// Case sensitive example:
///
/// ```
/// use librleri::*;
///
/// let hello = Keyword::new("hello", false);
/// let g = Grammar::new(&hello, None);
///
/// assert_eq!(g.parse("hello").is_valid(), true);
/// assert_eq!(g.parse("Hello").is_valid(), false);
/// ```
///
/// Case in-sensitive example:
///
/// ```
/// use librleri::*;
///
/// let hello = Keyword::new("hello", true);
/// let g = Grammar::new(&hello, None);
///
/// assert_eq!(g.parse("hello").is_valid(), true);
/// assert_eq!(g.parse("Hello").is_valid(), true);
/// ```
impl Keyword {
    pub fn new(keyword: &str, ignore_case: bool) -> Elem {
        let keyword = if ignore_case {
            keyword.to_lowercase()
        } else {
            String::from(keyword)
        };
        Rc::new(RefCell::new(Keyword {
            id: None,
            keyword,
            ignore_case,
        }))
    }

    pub fn with_id(id: i32, keyword: &str, ignore_case: bool) -> Elem {
        Rc::new(RefCell::new(Keyword {
            id: Some(id),
            keyword: String::from(keyword),
            ignore_case,
        }))
    }
}

impl Element for Keyword {
    fn id(&self) -> Option<i32> {
        self.id
    }

    fn parse(&self, this: &Elem, parser: &mut Parser, parent: &mut Node) -> bool {
        let mat = match parser.kw_match(parent.pos + parent.len) {
            Some(kw) => kw,
            None => "no_match",
        };

        if (self.ignore_case && mat.to_lowercase() == self.keyword)
            || (!self.ignore_case && mat == self.keyword)
        {
            let node = Node::new(parent.pos, mat.len(), Rc::clone(this));
            parent.len += node.len;
            parent.children.push(node);
            return true;
        }
        false
    }

    fn kind(&self) -> Kind {
        Kind::Keyword(self)
    }

    fn kind_mut(&mut self) -> Kind {
        Kind::Keyword(self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[macro_export]
macro_rules! keyword {
    ( $keyword:expr ) => {
        crate::element::keyword::Keyword::new($keyword, false)
    };

    ( $keyword:expr, $ignore_case:expr ) => {
        crate::element::keyword::Keyword::new($keyword, $ignore_case)
    };

    ( id=$id:expr; $keyword:expr ) => {
        crate::element::keyword::Keyword::with_id($id, $keyword, false)
    };

    ( id=$id:expr; $keyword:expr, $ignore_case:expr ) => {
        crate::element::keyword::Keyword::with_id($id, $keyword, $ignore_case)
    };
}
