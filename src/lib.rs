mod element;
mod parsing;

pub use crate::element::forward::Forward;
pub use crate::element::keyword::Keyword;
pub use crate::element::sequence::Sequence;
pub use crate::element::Elem;
pub use crate::element::Element;
pub use crate::element::Kind;
pub use crate::parsing::grammar::Grammar;

#[cfg(test)]
mod tests {
    use super::*;
    use matches::assert_matches;

    #[test]
    fn keyword_macro() {
        let hi = keyword!("hi");
        let hi_with_id = keyword!(id=42; "hi");
        let hi_case_insensitive = keyword!("HI", true);

        assert_eq!(hi.borrow().id(), None);
        assert_eq!(hi_with_id.borrow().id(), Some(42));
        assert_matches!(hi.borrow().kind(), Kind::Keyword(_));
        assert_eq!(Grammar::new(&hi, None).parse("hi").is_valid(), true);
        assert_eq!(Grammar::new(&hi, None).parse("Hi").is_valid(), false);
        assert_eq!(Grammar::new(&hi_case_insensitive, None).parse("hi").is_valid(), true);
        assert_eq!(Grammar::new(&hi_case_insensitive, None).parse("Hi").is_valid(), true);
    }

    #[test]
    fn sequence_macro() {
        let seq = sequence!(&keyword!("hello"), &keyword!("world"));

        assert_eq!(seq.borrow().id(), None);
        assert_matches!(seq.borrow().kind(), Kind::Sequence(_));
        assert_eq!(Grammar::new(&seq, None).parse("hello  world  ").is_valid(), true);
        assert_eq!(Grammar::new(&seq, None).parse("hello  world! ").is_valid(), false);
    }

    #[test]
    fn forward_macro() {
        let fwd = forward!();
        let seq = sequence!(&keyword!("hello"), &fwd);
        forward_set!(fwd, &keyword!(id=42; "world"));

        assert_eq!(fwd.borrow().id(), Some(42));
        assert_eq!(Grammar::new(&seq, None).parse("hello  world  ").is_valid(), true);
        assert_eq!(Grammar::new(&seq, None).parse("hello  world! ").is_valid(), false);
    }

}
