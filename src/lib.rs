pub mod and;
pub mod debug;
pub mod many;
pub mod option;
pub mod or;
pub mod transform;

pub trait PIterator: Clone + Iterator {}
#[derive(Debug)]
pub struct PError;
impl std::fmt::Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error")
    }
}
impl std::error::Error for PError {}

pub type PResult<O, I> = std::result::Result<(I, O), PError>;

impl<'a> PIterator for std::str::Chars<'a> {}

#[macro_export]
macro_rules! parser_builder {
    ($parser_iter:ty; Iter) => {
        |mut input: $parser_iter| {
            let next = input.next().ok_or(PError)?;
            Ok((input, next))
        }
    };
    ($parser_iter:ty; Iter $v:expr) => {
        |mut input: $parser_iter| {
            let next = input.next().ok_or(PError)?;
            if next == $v {
                Ok((input, next))
            } else {
                Err(PError)
            }
        }
    };
    ($parser_iter:ty; $v:ident) => {
        $v()
    };
    ($parser_iter:ty; ($($p:tt)*)) => {
        parser_builder!($parser_iter; $($p)*)
    };
    ($parser_iter:ty; ($($p1:tt)*) | ($($p2:tt)*)) => {
        $crate::or::or2(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) | ($($p2:tt)*) | ($($p3:tt)*)) => {
        $crate::or::or3(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) | ($($p2:tt)*) | ($($p3:tt)*) | ($($p4:tt)*)) => {
        $crate::or::or4(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) | ($($p2:tt)*) | ($($p3:tt)*) | ($($p4:tt)*) | ($($p5:tt)*)) => {
        $crate::or::or5(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) | ($($p2:tt)*) | ($($p3:tt)*) | ($($p4:tt)*) | ($($p5:tt)*) | ($($p6:tt)*)) => {
        $crate::or::or6(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
            parser_builder!($parser_iter; $($p6)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) | ($($p2:tt)*) | ($($p3:tt)*) | ($($p4:tt)*) | ($($p5:tt)*) | ($($p6:tt)*) | ($($p7:tt)*)) => {
        $crate::or::or7(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
            parser_builder!($parser_iter; $($p6)*),
            parser_builder!($parser_iter; $($p7)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*)) => {
        $crate::and::and2(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*) > ($($p3:tt)*)) => {
        $crate::and::and3(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*) > ($($p3:tt)*) > ($($p4:tt)*)) => {
        $crate::and::and4(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*) > ($($p3:tt)*) > ($($p4:tt)*) > ($($p5:tt)*)) => {
        $crate::and::and5(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*) > ($($p3:tt)*) > ($($p4:tt)*) > ($($p5:tt)*) > ($($p6:tt)*)) => {
        $crate::and::and6(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
            parser_builder!($parser_iter; $($p6)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*) > ($($p3:tt)*) > ($($p4:tt)*) > ($($p5:tt)*) > ($($p6:tt)*) > ($($p7:tt)*)) => {
        $crate::and::and7(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
            parser_builder!($parser_iter; $($p6)*),
            parser_builder!($parser_iter; $($p7)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*) > ($($p3:tt)*) > ($($p4:tt)*) > ($($p5:tt)*) > ($($p6:tt)*) > ($($p7:tt)*) > ($($p8:tt)*)) => {
        $crate::and::and8(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
            parser_builder!($parser_iter; $($p6)*),
            parser_builder!($parser_iter; $($p7)*),
            parser_builder!($parser_iter; $($p8)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) > ($($p2:tt)*) > ($($p3:tt)*) > ($($p4:tt)*) > ($($p5:tt)*) > ($($p6:tt)*) > ($($p7:tt)*) > ($($p8:tt)*) > ($($p9:tt)*)) => {
        $crate::and::and9(
            parser_builder!($parser_iter; $($p1)*),
            parser_builder!($parser_iter; $($p2)*),
            parser_builder!($parser_iter; $($p3)*),
            parser_builder!($parser_iter; $($p4)*),
            parser_builder!($parser_iter; $($p5)*),
            parser_builder!($parser_iter; $($p6)*),
            parser_builder!($parser_iter; $($p7)*),
            parser_builder!($parser_iter; $($p8)*),
            parser_builder!($parser_iter; $($p9)*),
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) starts with ($($p2:tt)*)) => {
        $crate::transform::map(
            $crate::and::and2(
                parser_builder!($parser_iter; $($p2)*),
                parser_builder!($parser_iter; $($p1)*),
            ),
            |(_, v)| v
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) ends with ($($p2:tt)*)) => {
        $crate::transform::map(
            $crate::and::and2(
                parser_builder!($parser_iter; $($p1)*),
                parser_builder!($parser_iter; $($p2)*),
            ),
            |(v, _)| v
        )
    };
    ($parser_iter:ty; ($($p1:tt)*) surrounded by ($($p2:tt)*) and ($($p3:tt)*)) => {
        $crate::transform::map(
            $crate::and::and3(
                parser_builder!($parser_iter; $($p2)*),
                parser_builder!($parser_iter; $($p1)*),
                parser_builder!($parser_iter; $($p3)*),
            ),
            |(_, v, _)| v
        )
    };
    ($parser_iter:ty; map ($($p:tt)*) as $($closure:tt)*) => {
        $crate::transform::map(parser_builder!($parser_iter; $($p)*), $($closure)*)
    };
    ($parser_iter:ty; flatmap ($($p:tt)*) as $($closure:tt)*) => {
        $crate::transform::flatmap(parser_builder!($parser_iter; $($p)*), $($closure)*)
    };
    ($parser_iter:ty; ($($p:tt)*) ?) => {
        $crate::option::option(parser_builder!($parser_iter; $($p)*))
    };
    ($parser_iter:ty; ($($p:tt)*) *) => {
        $crate::many::many(parser_builder!($parser_iter; $($p)*))
    };
    ($parser_iter:ty; ($($p:tt)*) +) => {
        $crate::many::at_least_one(parser_builder!($parser_iter; $($p)*))
    };
    ($parser_iter:ty; debug ($($p:tt)*)) => {
        $crate::debug::debug(parser_builder!($parser_iter; $($p)*))
    };
    ($parser_iter:ty; ($($p:tt)*) while $($closure:tt)*) => {
        $crate::many::take_while($($closure)*, parser_builder!($parser_iter; $($p)*))
    };
}

#[macro_export]
macro_rules! parser {
    (
        $parser_iter:ty => $res_enum_name:ident;

        $enum_varient:ident : { $($rule:tt)* };

        $($rest:tt)*
    ) => {
        #[allow(non_snake_case)]
        pub fn $enum_varient<'a>() -> impl Fn($parser_iter) -> PResult<$res_enum_name, $parser_iter>
        {
            $crate::transform::map(parser_builder!($parser_iter; $($rule)*), |_| $res_enum_name::$enum_varient)
        }
        parser!{
            $parser_iter => $res_enum_name;
            $($rest)*
        }
    };
    (
        $parser_iter:ty => $res_enum_name:ident;

        nomap $enum_varient:ident : { $($rule:tt)* };

        $($rest:tt)*
    ) => {
        #[allow(non_snake_case)]
        pub fn $enum_varient<'a>() -> impl Fn($parser_iter) -> PResult<$res_enum_name, $parser_iter>
        {
            parser_builder!($parser_iter; $($rule)*)
        }
        parser!{
            $parser_iter => $res_enum_name;
            $($rest)*
        }
    };
    (
        $parser_iter:ty => $res_enum_name:ident;

        $enum_varient:ident as $as_type:ty : { $($rule:tt)* };

        $($rest:tt)*
    ) => {
        #[allow(non_snake_case)]
        pub fn $enum_varient<'a>() -> impl Fn($parser_iter) -> PResult<$as_type, $parser_iter>
        {
            parser_builder!($parser_iter; $($rule)*)
        }
        parser!{
            $parser_iter => $res_enum_name;
            $($rest)*
        }
    };
    (
        $parser_iter:ty => $res_enum_name:ident;

        recursive $enum_varient:ident : { $($rule:tt)* };

        $($rest:tt)*
    ) => {
        #[allow(non_snake_case)]
        pub fn $enum_varient<'a>() -> impl Fn($parser_iter) -> PResult<$res_enum_name, $parser_iter>
        {
            |input: $parser_iter| {
                parser_builder!($parser_iter; $($rule)*)(input)
            }
        }
        parser!{
            $parser_iter => $res_enum_name;
            $($rest)*
        }
    };
    (
        $parser_iter:ty => $res_enum_name:ident;
    ) => {};
}
