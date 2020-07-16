pub mod and;
pub mod many;
pub mod or;
pub mod transform;

pub trait PIterator: Clone + Iterator {}
#[derive(Debug)]
pub struct PError;
pub type PResult<O, I> = std::result::Result<(I, O), PError>;

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
    ($parser_iter:ty; ($($p:tt)*) *) => {
        $crate::many::many(parser_builder!($parser_iter; $($p)*))
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

        $enum_varient:ident : { $($rule:tt)* } map {$($closure:tt)*};

        $($rest:tt)*
    ) => {
        #[allow(non_snake_case)]
        pub fn $enum_varient<'a>() -> impl Fn($parser_iter) -> PResult<$res_enum_name, $parser_iter>
        {
            $crate::transform::map(parser_builder!($parser_iter; $($rule)*), $($closure)*)
        }
        parser!{
            $parser_iter => $res_enum_name;
            $($rest)*
        }
    };
    (
        $parser_iter:ty => $res_enum_name:ident;

        $enum_varient:ident : { $($rule:tt)* } nomap;

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

#[test]
fn test() {
    impl<'a> PIterator for std::str::Chars<'a> {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        Plus,
        Minus,
        BracketOpen,
        BracketClose,
        ChevronLeft,
        ChevronRight,
        Tokens(Vec<Token>),
    }

    parser! {
        std::str::Chars<'a> => Token;

        Plus : { Iter '+' };
        Minus : { Iter '-' };
        BracketOpen : { Iter '[' };
        BracketClose : { Iter ']' };
        ChevronLeft : { Iter '>' };
        ChevronRight : { Iter '<' };
        Tokens: { ((Plus) | (Minus) | (BracketOpen) | (BracketClose) | (ChevronLeft) | (ChevronRight))* } map { |v| Token::Tokens(v) };
    }

    #[derive(Debug)]
    pub enum Node {
        Inc,
        Dec,
        PtrP,
        PtrM,
        Loop(Vec<Node>),
        Nodes(Vec<Node>),
    }

    #[derive(Debug, Clone)]
    pub struct Tokens<'a> {
        v: std::slice::Iter<'a, Token>,
    }
    impl<'a> Tokens<'a> {
        pub fn new(v: &'a Vec<Token>) -> Self {
            Tokens { v: v.iter() }
        }
    }
    impl<'a> Iterator for Tokens<'a> {
        type Item = Token;
        fn next(&mut self) -> Option<Token> {
            self.v.next().to_owned().map(|v| v.clone())
        }
    }
    impl<'a> PIterator for Tokens<'a> {}

    parser! {
        Tokens<'a> => Node;
        Inc: { Iter Token::Plus };
        Dec: { Iter Token::Minus };
        PtrP: { Iter Token::ChevronRight };
        PtrM: { Iter Token::ChevronLeft };
        Action: { (Inc) | (Dec) | (PtrP) | (PtrM) } nomap;
        recursive Loop: { (map (((Action) | (Loop))*) as (|v| Node::Loop(v))) surrounded by (Iter Token::BracketOpen) and (Iter Token::BracketClose) };
        Nodes: { ((Action) | (Loop))* } map { |v| Node::Nodes(v) };
    }

    let input = "+>[+-[+>]>]".chars();

    let (input, r) = Tokens()(input).unwrap();
    println!("{:?}", r);
    if let Token::Tokens(v) = r {
        let input = Tokens::new(&v);
        let (input, r) = Nodes()(input).unwrap();
        println!("{:?}", r);
    }
}
