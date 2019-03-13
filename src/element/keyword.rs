use std::fmt;
use std::rc::Rc;

use super::{Element, Kind};
use crate::parsing::node::Node;
use crate::parsing::parser::Parser;

/// A `Keyword` element.
///
/// Keywords must match a grammar.
///
/// # Examples
///
/// Case sensitive example:
///
/// ```
/// use libleri::{Keyword, Grammar};
///
/// let hello = keyword!("hello");
/// let g = grammar!(&hello);
///
/// assert_eq!(parse(grammar, "hello").is_valid, true);
/// assert_eq!(parse(grammar, "Hello").is_valid, false);
/// ```
///
/// Case in-sensitive example:
///
/// ```
/// use libleri::{keyword, Keyword, Grammar};
///
/// let hello = keyword!("hello", true);
/// let g = Grammar::new(&hello);
///
/// assert_eq!(g.parse("hello").is_valid(), true);
/// assert_eq!(g.parse("Hello").is_valid(), true);
/// ```
#[derive(Debug)]
pub struct Keyword {
    id: Option<i32>,
    keyword: String,
    ignore_case: bool,
}

impl Keyword {
    pub fn new(keyword: &str, ignore_case: bool) -> Rc<dyn Element> {
        let keyword = if ignore_case {
            keyword.to_lowercase()
        } else {
            String::from(keyword)
        };
        Rc::new(Keyword {
            id: None,
            keyword,
            ignore_case,
        })
    }

    pub fn with_id(id: i32, keyword: &str, ignore_case: bool) -> Rc<dyn Element> {
        Rc::new(Keyword {
            id: Some(id),
            keyword: String::from(keyword),
            ignore_case,
        })
    }
}

impl Element for Keyword {
    fn id(&self) -> Option<i32> {
        self.id
    }

    fn parse(&self, parser: &Parser) -> Result<Node, &'static str> {
        Err("not implemented")
    }

    fn kind(&self) -> Kind {
        Kind::Keyword
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

    ( $id:expr; $keyword:expr ) => {
        crate::element::keyword::Keyword::with_id($id, $keyword, false)
    };

    ( $id:expr; $keyword:expr, $ignore_case:expr ) => {
        crate::element::keyword::Keyword::with_id($id, $keyword, $ignore_case)
    };
}
