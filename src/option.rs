use crate::{PIterator, PResult};

pub fn option<O, I, F>(parser: F) -> impl Fn(I) -> PResult<Option<O>, I>
where
    I: PIterator,
    F: Fn(I) -> PResult<O, I>,
{
    move |input: I| match parser(input.clone()) {
        Ok((input, v)) => Ok((input, Some(v))),
        Err(_) => Ok((input, None)),
    }
}
