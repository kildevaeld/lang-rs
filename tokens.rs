#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use lang_lexing::tokens::{Ident, Literal, LiteralNumber};
use lang_parsing::Parser;
#[macro_use]
pub mod tokens {
    use super::*;
    pub type Extract<'input> = (
        LiteralNumber,
        lang::lexing::tokens::Punct<'input>,
        lang::lexing::tokens::Ident<'input>,
    );
    pub type Lexer<'input> = lang::lexing::Lexer<
        'input,
        Extract<'input>,
        lang::lexing::tokens::Token<'input>,
    >;
    pub struct Func {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Func {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Func",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Func {
        #[inline]
        fn clone(&self) -> Func {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Func {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Func
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::keyword_peek(cursor, "fn")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Func
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::keyword(input, "fn")?.span;
            Ok(Func { span })
        }
    }
    impl lang::lexing::WithSpan for Func {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Add {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Add {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Add",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Add {
        #[inline]
        fn clone(&self) -> Add {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Add {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Add
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "+")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Add
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "+")?;
            Ok(Add { span })
        }
    }
    impl lang::lexing::WithSpan for Add {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Sub {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Sub {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Sub",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Sub {
        #[inline]
        fn clone(&self) -> Sub {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Sub {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Sub
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "-")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Sub
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "-")?;
            Ok(Sub { span })
        }
    }
    impl lang::lexing::WithSpan for Sub {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Mul {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Mul {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Mul",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Mul {
        #[inline]
        fn clone(&self) -> Mul {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Mul {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Mul
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "*")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Mul
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "*")?;
            Ok(Mul { span })
        }
    }
    impl lang::lexing::WithSpan for Mul {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Div {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Div {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Div",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Div {
        #[inline]
        fn clone(&self) -> Div {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Div {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Div
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "/")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Div
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "/")?;
            Ok(Div { span })
        }
    }
    impl lang::lexing::WithSpan for Div {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Equal {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Equal {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Equal",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Equal {
        #[inline]
        fn clone(&self) -> Equal {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Equal {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Equal
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "==")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Equal
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "==")?;
            Ok(Equal { span })
        }
    }
    impl lang::lexing::WithSpan for Equal {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct OpenParens {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OpenParens {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "OpenParens",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OpenParens {
        #[inline]
        fn clone(&self) -> OpenParens {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OpenParens {}
    impl<'input, T> lang::parsing::Peek<'input, T> for OpenParens
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "(")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for OpenParens
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "(")?;
            Ok(OpenParens { span })
        }
    }
    impl lang::lexing::WithSpan for OpenParens {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct CloseParens {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CloseParens {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "CloseParens",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CloseParens {
        #[inline]
        fn clone(&self) -> CloseParens {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CloseParens {}
    impl<'input, T> lang::parsing::Peek<'input, T> for CloseParens
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, ")")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for CloseParens
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, ")")?;
            Ok(CloseParens { span })
        }
    }
    impl lang::lexing::WithSpan for CloseParens {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct OpenBrace {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OpenBrace {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "OpenBrace",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OpenBrace {
        #[inline]
        fn clone(&self) -> OpenBrace {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OpenBrace {}
    impl<'input, T> lang::parsing::Peek<'input, T> for OpenBrace
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "{")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for OpenBrace
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "{")?;
            Ok(OpenBrace { span })
        }
    }
    impl lang::lexing::WithSpan for OpenBrace {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct CloseBrace {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CloseBrace {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "CloseBrace",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CloseBrace {
        #[inline]
        fn clone(&self) -> CloseBrace {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CloseBrace {}
    impl<'input, T> lang::parsing::Peek<'input, T> for CloseBrace
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "}")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for CloseBrace
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "}")?;
            Ok(CloseBrace { span })
        }
    }
    impl lang::lexing::WithSpan for CloseBrace {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Comma {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Comma {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Comma",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comma {
        #[inline]
        fn clone(&self) -> Comma {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Comma {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Comma
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, ",")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Comma
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, ",")?;
            Ok(Comma { span })
        }
    }
    impl lang::lexing::WithSpan for Comma {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Assign {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Assign {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Assign",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Assign {
        #[inline]
        fn clone(&self) -> Assign {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Assign {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Assign
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "=")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Assign
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "=")?;
            Ok(Assign { span })
        }
    }
    impl lang::lexing::WithSpan for Assign {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
}
pub enum BinaryOperator {
    Sub,
    Add,
    Div,
    Mul,
    Assign,
    Eq,
}
#[automatically_derived]
impl ::core::fmt::Debug for BinaryOperator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                BinaryOperator::Sub => "Sub",
                BinaryOperator::Add => "Add",
                BinaryOperator::Div => "Div",
                BinaryOperator::Mul => "Mul",
                BinaryOperator::Assign => "Assign",
                BinaryOperator::Eq => "Eq",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BinaryOperator {
    #[inline]
    fn clone(&self) -> BinaryOperator {
        match self {
            BinaryOperator::Sub => BinaryOperator::Sub,
            BinaryOperator::Add => BinaryOperator::Add,
            BinaryOperator::Div => BinaryOperator::Div,
            BinaryOperator::Mul => BinaryOperator::Mul,
            BinaryOperator::Assign => BinaryOperator::Assign,
            BinaryOperator::Eq => BinaryOperator::Eq,
        }
    }
}
pub enum Expr<'a> {
    Lit(Literal<'a>),
    Binary { left: Box<Expr<'a>>, right: Box<Expr<'a>>, op: BinaryOperator },
    Ident(Ident<'a>),
}
#[automatically_derived]
impl<'a> ::core::fmt::Debug for Expr<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Expr::Lit(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lit", &__self_0)
            }
            Expr::Binary { left: __self_0, right: __self_1, op: __self_2 } => {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Binary",
                    "left",
                    __self_0,
                    "right",
                    __self_1,
                    "op",
                    &__self_2,
                )
            }
            Expr::Ident(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ident", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for Expr<'a> {
    #[inline]
    fn clone(&self) -> Expr<'a> {
        match self {
            Expr::Lit(__self_0) => Expr::Lit(::core::clone::Clone::clone(__self_0)),
            Expr::Binary { left: __self_0, right: __self_1, op: __self_2 } => {
                Expr::Binary {
                    left: ::core::clone::Clone::clone(__self_0),
                    right: ::core::clone::Clone::clone(__self_1),
                    op: ::core::clone::Clone::clone(__self_2),
                }
            }
            Expr::Ident(__self_0) => Expr::Ident(::core::clone::Clone::clone(__self_0)),
        }
    }
}
#[allow(unused_braces, non_snake_case)]
mod expression {
    use super::*;
    use lang::{
        parsing::{Parse, Peek, TokenReader, Cursor, Error},
        lexing::{tokens::Token, WithSpan, Span},
    };
    fn __peek<'input>(input: &mut Cursor<'input, '_, Token<'input>>) -> bool {
        input.peek::<Literal>() || input.peek::<Ident>()
    }
    fn __get_precedence<'input>(
        input: &mut TokenReader<'input, '_, Token<'input>>,
    ) -> u8 {
        if (input.peek_offset::<crate::tokens::Assign>(0usize)
            && !input.peek_offset::<crate::tokens::Assign>(1usize))
        {
            1u8
        } else if input.peek_offset::<crate::tokens::Equal>(0usize) {
            2u8
        } else if input.peek_offset::<crate::tokens::Add>(0usize)
            || input.peek_offset::<crate::tokens::Sub>(0usize)
        {
            3u8
        } else if input.peek_offset::<crate::tokens::Div>(0usize)
            || input.peek_offset::<crate::tokens::Mul>(0usize)
        {
            4u8
        } else {
            0u8
        }
    }
    fn __prefix<'input>(
        input: &mut TokenReader<'input, '_, Token<'input>>,
    ) -> Result<Expr<'input>, Error> {
        if input.peek_offset::<Literal>(0usize) {
            let o = input.parse::<Literal>()?;
            { Ok(Expr::Lit(o)) }
        } else if input.peek_offset::<Ident>(0usize) {
            let i = input.parse::<Ident>()?;
            { Ok(Expr::Ident(i)) }
        } else {
            ::core::panicking::panic_fmt(format_args!(""))
        }
    }
    fn __infix<'input>(
        input: &mut TokenReader<'input, '_, Token<'input>>,
        left: Expr<'input>,
    ) -> Result<Expr<'input>, Error> {
        if (input.peek_offset::<crate::tokens::Assign>(0usize)
            && !input.peek_offset::<crate::tokens::Assign>(1usize))
        {
            let lhs = left;
            let _ = input.parse::<crate::tokens::Assign>()?;
            let rhs = __expression(input, 1u8)?;
            {
                Ok(Expr::Binary {
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                    op: BinaryOperator::Assign,
                })
            }
        } else if input.peek_offset::<crate::tokens::Equal>(0usize) {
            let lhs = left;
            let _ = input.parse::<crate::tokens::Equal>()?;
            let rhs = __expression(input, 2u8)?;
            {
                Ok(Expr::Binary {
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                    op: BinaryOperator::Eq,
                })
            }
        } else if input.peek_offset::<crate::tokens::Add>(0usize)
            || input.peek_offset::<crate::tokens::Sub>(0usize)
        {
            let lhs = left;
            let op = if input.peek_offset::<crate::tokens::Add>(0usize) {
                let _ = input.parse::<crate::tokens::Add>()?;
                { BinaryOperator::Add }
            } else if input.peek_offset::<crate::tokens::Sub>(0usize) {
                let _ = input.parse::<crate::tokens::Sub>()?;
                { BinaryOperator::Sub }
            } else {
                ::core::panicking::panic_fmt(format_args!(""))
            };
            let rhs = __expression(input, 3u8)?;
            {
                Ok(Expr::Binary {
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                    op,
                })
            }
        } else if input.peek_offset::<crate::tokens::Div>(0usize)
            || input.peek_offset::<crate::tokens::Mul>(0usize)
        {
            let lhs = left;
            let op = if input.peek_offset::<crate::tokens::Div>(0usize) {
                let _ = input.parse::<crate::tokens::Div>()?;
                { BinaryOperator::Div }
            } else if input.peek_offset::<crate::tokens::Mul>(0usize) {
                let _ = input.parse::<crate::tokens::Mul>()?;
                { BinaryOperator::Mul }
            } else {
                ::core::panicking::panic_fmt(format_args!(""))
            };
            let rhs = __expression(input, 4u8)?;
            {
                Ok(Expr::Binary {
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                    op,
                })
            }
        } else {
            ::core::panicking::panic_fmt(format_args!(""))
        }
    }
    fn __expression<'input>(
        input: &mut TokenReader<'input, '_, Token<'input>>,
        precedence: u8,
    ) -> Result<Expr<'input>, Error> {
        let mut left = __prefix(input)?;
        {
            ::std::io::_print(
                format_args!(
                    "precedence: {0} - {1}\n", precedence, __get_precedence(input)
                ),
            );
        };
        while precedence < __get_precedence(input) {
            left = __infix(input, left)?;
        }
        Ok(left)
    }
    pub fn parse<'input>(
        input: &mut TokenReader<'input, '_, Token<'input>>,
    ) -> Result<Expr<'input>, Error> {
        __expression(input, 0)
    }
    impl<'input> Parse<'input, Token<'input>> for Expr<'input> {
        fn parse(
            input: &mut TokenReader<'input, '_, Token<'input>>,
        ) -> Result<Expr<'input>, Error> {
            parse(input)
        }
    }
    impl<'input> Peek<'input, Token<'input>> for Expr<'input> {
        fn peek(cursor: &mut Cursor<'input, '_, Token<'input>>) -> bool {
            __peek(cursor)
        }
    }
}
fn main() {
    let input = "ident = 20 - 100 / 2 == 202";
    let lexer = tokens::Lexer::new(input);
    let mut parser = Parser::from_tokens(input, lexer.skip_whitespace(true).tokenize())
        .expect("lex");
    let ast = parser.parse::<Expr>().expect("message");
    {
        ::std::io::_print(format_args!("{0:#?}\n", ast));
    }
}
