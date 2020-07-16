use crate::{PIterator, PResult};

pub fn map<O1, O2, I, F, G>(parser: F, mapper: G) -> impl Fn(I) -> PResult<O2, I>
where
    I: PIterator,
    F: Fn(I) -> PResult<O1, I>,
    G: Fn(O1) -> O2,
{
    move |input: I| parser(input).map(|(input, v): (I, O1)| (input, mapper(v)))
}
