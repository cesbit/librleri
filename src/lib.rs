mod element;
mod parsing;

pub use crate::element::Element;
pub use crate::element::keyword::Keyword;
pub use crate::element::sequence::Sequence;
pub use crate::parsing::grammar::Grammar;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_macro() {
        let hi = keyword!("hi");
        let seq1 = sequence![5; &hi, &hi];
        let seq2 = sequence![&seq1, &hi];

        let g = Grammar::new(seq2, None);

        let res = g.parse("hi Iris!");

        println!("Is valid: {}", res.is_valid());

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

        println!("{:?}", g);
    }
}
