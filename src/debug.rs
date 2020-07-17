use crate::{PIterator, PResult};

pub fn debug<O, I, F>(parser: F) -> impl Fn(I) -> PResult<O, I>
where
    I: PIterator + std::fmt::Debug,
    O: std::fmt::Debug,
    F: Fn(I) -> PResult<O, I>,
{
    move |input: I| {
        println!("Parsing {:?}", input);
        parser(input)
            .map(|v| {
                println!("Success: {:?}\n", v.1);
                v
            })
            .map_err(|v| {
                println!("Failure\n");
                v
            })
    }
}
