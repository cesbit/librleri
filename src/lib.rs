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
        assert_eq!(Grammar::new(&seq, None).parse("hello  world").is_valid(), true);
    }
        // let seq1 = sequence![5; &hi, &keyword!("hello"), &fwd];
        // let seq2 = sequence![&seq1, &hi, &fwd];

        // {
        //     let mut rfwd = fwd.borrow_mut();
        //     rfwd.as_mut_forward().unwrap().set_forward(&seq1);
        // }

        // let g = Grammar::new(&seq2, None);

        // let res = g.parse("hi Iris!");

        // println!("Is valid: {}", res.is_valid());

        // let hi_ptr: *const Element = std::rc::Rc::into_raw(hi);

        // std::rc::Rc::try_unwrap(hi)
        // do_work(hi.downcast_ref());

        // let x = vec![1, 2, 3];

        // let res = Rc::try_unwrap(hi);
        // match res {

        // }

        // match weak_five.as_any() {}

        // let hi_ptr: *const Element = Rc::into_raw(hi);

        // match hi.as_any() {
        //     Keyword<> => println!("Keyword..."),
        //     _ => println!("Other..."),
        // }

        // let g = Grammar::new(Box::new(&seq2));

        // println!("{:?}", g);
}
