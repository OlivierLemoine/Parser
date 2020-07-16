use crate::{PIterator, PResult};

pub fn take_while<O, I, F, G>(predicate: G, parser: F) -> impl Fn(I) -> PResult<Vec<O>, I>
where
    I: PIterator,
    F: Fn(I) -> PResult<O, I>,
    G: Fn(&O) -> bool,
{
    move |mut input: I| {
        let mut res = vec![];

        loop {
            if let Ok((new_input, item)) = parser(input.clone()) {
                if !predicate(&item) {
                    break;
                }
                input = new_input;
                res.push(item);
            } else {
                break;
            }
        }

        Ok((input, res))
    }
}

pub fn many<O, I, F>(parser: F) -> impl Fn(I) -> PResult<Vec<O>, I>
where
    I: PIterator,
    F: Fn(I) -> PResult<O, I>,
{
    take_while(|_| true, parser)
}
