use crate::{PIterator, PResult};

macro_rules! impl_and {
    ($fn_name:ident; $($param_name:ident: $param_type:ident > $output_type:ident), *) => {
        pub fn $fn_name<O1, I, F1, $($param_type,)* $($output_type),*>(parser1: F1, $($param_name: $param_type),*) -> impl Fn(I) -> PResult<(O1, $($output_type),*), I>
        where
            I: PIterator,
            F1: Fn(I) -> PResult<O1, I>,
            $($param_type: Fn(I) -> PResult<$output_type, I>,)*
        {
            move |input: I| {
                let (input, parser1) = parser1(input)?;
                $(let (input, $param_name) = $param_name(input)?;)*
                Ok((input, (parser1, $($param_name),*)))
            }
        }
    };
}

impl_and! {and2; parser2: F2 > O2}
impl_and! {and3; parser2: F2 > O2, parser3: F3 > O3}
impl_and! {and4; parser2: F2 > O2, parser3: F3 > O3, parser4: F4 > O4}
impl_and! {and5; parser2: F2 > O2, parser3: F3 > O3, parser4: F4 > O4, parser5: F5 > O5}
impl_and! {and6; parser2: F2 > O2, parser3: F3 > O3, parser4: F4 > O4, parser5: F5 > O5, parser6: F6 > O6}
