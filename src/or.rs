use crate::{PIterator, PResult};

macro_rules! impl_or {
    ($fn_name:ident; $($param_name:ident: $param_type:ident), *) => {
        pub fn $fn_name<O, I, F1, $($param_type),*>(parser1: F1, $($param_name: $param_type),*) -> impl Fn(I) -> PResult<O, I>
        where
            I: PIterator,
            F1: Fn(I) -> PResult<O, I>,
            $($param_type: Fn(I) -> PResult<O, I>,)*
        {
            move |input: I| {
                parser1(input.clone())
                    $(.or($param_name(input.clone())))*
            }
        }
    };
}

impl_or! {or2; parser2: F2}
impl_or! {or3; parser2: F2, parser3: F3}
impl_or! {or4; parser2: F2, parser3: F3, parser4: F4}
impl_or! {or5; parser2: F2, parser3: F3, parser4: F4, parser5: F5}
impl_or! {or6; parser2: F2, parser3: F3, parser4: F4, parser5: F5, parser6: F6}
impl_or! {or7; parser2: F2, parser3: F3, parser4: F4, parser5: F5, parser6: F6, parser7: F7}
