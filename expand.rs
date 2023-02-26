#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use lang_parsing::Parser;
use stmt::Stmt;
#[macro_use]
mod tokens {
    use lang::lexing::tokens::{LiteralNumber, LiteralString};
    pub type Extract<'input> = (
        LiteralString,
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
    pub struct Let {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Let {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Let",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Let {
        #[inline]
        fn clone(&self) -> Let {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Let {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Let
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::keyword_peek(cursor, "let")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Let
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::keyword(input, "let")?.span;
            Ok(Let { span })
        }
    }
    impl lang::lexing::WithSpan for Let {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct For {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for For {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "For",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for For {
        #[inline]
        fn clone(&self) -> For {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for For {}
    impl<'input, T> lang::parsing::Peek<'input, T> for For
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::keyword_peek(cursor, "for")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for For
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::keyword(input, "for")?.span;
            Ok(For { span })
        }
    }
    impl lang::lexing::WithSpan for For {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct In {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for In {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "In",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for In {
        #[inline]
        fn clone(&self) -> In {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for In {}
    impl<'input, T> lang::parsing::Peek<'input, T> for In
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::keyword_peek(cursor, "in")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for In
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::keyword(input, "in")?.span;
            Ok(In { span })
        }
    }
    impl lang::lexing::WithSpan for In {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct If {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for If {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "If",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for If {
        #[inline]
        fn clone(&self) -> If {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for If {}
    impl<'input, T> lang::parsing::Peek<'input, T> for If
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::keyword_peek(cursor, "if")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for If
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::keyword(input, "if")?.span;
            Ok(If { span })
        }
    }
    impl lang::lexing::WithSpan for If {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Else {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Else {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Else",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Else {
        #[inline]
        fn clone(&self) -> Else {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Else {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Else
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::keyword_peek(cursor, "else")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Else
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::keyword(input, "else")?.span;
            Ok(Else { span })
        }
    }
    impl lang::lexing::WithSpan for Else {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
    pub struct Return {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Return {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Return",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Return {
        #[inline]
        fn clone(&self) -> Return {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Return {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Return
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::keyword_peek(cursor, "return")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Return
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Ident<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::keyword(input, "return")?.span;
            Ok(Return { span })
        }
    }
    impl lang::lexing::WithSpan for Return {
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
    pub struct Lte {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Lte {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Lte",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Lte {
        #[inline]
        fn clone(&self) -> Lte {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Lte {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Lte
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, "<=")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Lte
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, "<=")?;
            Ok(Lte { span })
        }
    }
    impl lang::lexing::WithSpan for Lte {
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
    pub struct Semi {
        pub span: lang::lexing::Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Semi {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Semi",
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Semi {
        #[inline]
        fn clone(&self) -> Semi {
            let _: ::core::clone::AssertParamIsClone<lang::lexing::Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Semi {}
    impl<'input, T> lang::parsing::Peek<'input, T> for Semi
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn peek(cursor: &mut lang::parsing::Cursor<'input, '_, T>) -> bool {
            lang::parsing::punctuation_peek(cursor, ";")
        }
    }
    impl<'input, T> lang::parsing::Parse<'input, T> for Semi
    where
        T: lang::lexing::TokenRef<lang::lexing::tokens::Punct<'input>>,
        T: lang::lexing::WithSpan,
    {
        fn parse(
            input: &mut lang::parsing::TokenReader<'input, '_, T>,
        ) -> Result<Self, lang::parsing::Error> {
            let span = lang::parsing::punctuation(input, ";")?;
            Ok(Semi { span })
        }
    }
    impl lang::lexing::WithSpan for Semi {
        fn span(&self) -> lang::lexing::Span {
            self.span
        }
    }
}
mod exprs {
    use lang::{
        parsing::{Group, Punctuated},
        visitor, Parse, Peek, WithSpan,
    };
    use lang_lexing::tokens::{Ident, Literal};
    pub enum BinaryOperator {
        Add(crate::tokens::Add),
        Sub(crate::tokens::Sub),
        Lte(crate::tokens::Lte),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BinaryOperator {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BinaryOperator::Add(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Add",
                        &__self_0,
                    )
                }
                BinaryOperator::Sub(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Sub",
                        &__self_0,
                    )
                }
                BinaryOperator::Lte(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Lte",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl<'parse> lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>>
    for BinaryOperator {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            if state.peek::<crate::tokens::Add>() {
                return Ok(BinaryOperator::Add(state.parse()?))
            } else if state.peek::<crate::tokens::Sub>() {
                return Ok(BinaryOperator::Sub(state.parse()?))
            } else if state.peek::<crate::tokens::Lte>() {
                return Ok(BinaryOperator::Lte(state.parse()?))
            }
            Err(
                state
                    .error((
                        "BinaryOperator",
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                lang::parsing::ErrorKind::Expected {
                                    message: "Add".into(),
                                    rule: Some("Add".into()),
                                },
                                lang::parsing::ErrorKind::Expected {
                                    message: "Sub".into(),
                                    rule: Some("Sub".into()),
                                },
                                lang::parsing::ErrorKind::Expected {
                                    message: "Lte".into(),
                                    rule: Some("Lte".into()),
                                },
                            ]),
                        ),
                    )),
            )
        }
    }
    impl lang::lexing::WithSpan for BinaryOperator {
        fn span(&self) -> lang::lexing::Span {
            match self {
                BinaryOperator::Add(field_0) => field_0.span(),
                BinaryOperator::Sub(field_0) => field_0.span(),
                BinaryOperator::Lte(field_0) => field_0.span(),
            }
        }
    }
    impl<'parse> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for BinaryOperator {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <crate::tokens::Add as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
                || <crate::tokens::Sub as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
                || <crate::tokens::Lte as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
        }
    }
    pub struct IdentExpr<'a> {
        pub ident: Ident<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for IdentExpr<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "IdentExpr",
                "ident",
                &&self.ident,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>>
    for IdentExpr<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(IdentExpr { ident: state.parse()? })
        }
    }
    impl<'a> lang::lexing::WithSpan for IdentExpr<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.ident.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for IdentExpr<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <Ident<
                '_,
            > as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub struct BinaryExpr<'a> {
        pub left: Box<Expr<'a>>,
        pub operator: BinaryOperator,
        pub right: Box<Expr<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BinaryExpr<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "BinaryExpr",
                "left",
                &&self.left,
                "operator",
                &&self.operator,
                "right",
                &&self.right,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>>
    for BinaryExpr<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(BinaryExpr {
                left: state.parse()?,
                operator: state.parse()?,
                right: state.parse()?,
            })
        }
    }
    impl<'a> lang::lexing::WithSpan for BinaryExpr<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.left.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for BinaryExpr<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <Box<
                Expr<'_>,
            > as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub type Args<'a> = Group<
        crate::tokens::OpenParens,
        Punctuated<Expr<'a>, crate::tokens::Comma>,
        crate::tokens::CloseParens,
    >;
    pub struct CallExpr<'a> {
        pub target: Box<Expr<'a>>,
        pub args: Args<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for CallExpr<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CallExpr",
                "target",
                &&self.target,
                "args",
                &&self.args,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>>
    for CallExpr<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(CallExpr {
                target: state.parse()?,
                args: state.parse()?,
            })
        }
    }
    impl<'a> lang::lexing::WithSpan for CallExpr<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.target.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for CallExpr<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <Box<
                Expr<'_>,
            > as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub enum Expr<'a> {
        Literal(Literal<'a>),
        Ident(IdentExpr<'a>),
        Binary(BinaryExpr<'a>),
        Call(CallExpr<'a>),
    }
    pub trait ExprVisitor<'a> {
        type Output;
        fn visit_literal_expr(&mut self, member: &Literal<'a>) -> Self::Output;
        fn visit_ident_expr(&mut self, member: &IdentExpr<'a>) -> Self::Output;
        fn visit_binary_expr(&mut self, member: &BinaryExpr<'a>) -> Self::Output;
        fn visit_call_expr(&mut self, member: &CallExpr<'a>) -> Self::Output;
    }
    impl<'a> Expr<'a> {
        pub fn accept<V: ExprVisitor<'a>>(&self, visitor: &mut V) -> V::Output {
            match self {
                Self::Literal(field_0) => visitor.visit_literal_expr(field_0),
                Self::Ident(field_0) => visitor.visit_ident_expr(field_0),
                Self::Binary(field_0) => visitor.visit_binary_expr(field_0),
                Self::Call(field_0) => visitor.visit_call_expr(field_0),
            }
        }
    }
    pub trait ExprVisitorMut<'a> {
        type Output;
        fn visit_mut_literal_expr(&mut self, member: &mut Literal<'a>) -> Self::Output;
        fn visit_mut_ident_expr(&mut self, member: &mut IdentExpr<'a>) -> Self::Output;
        fn visit_mut_binary_expr(&mut self, member: &mut BinaryExpr<'a>) -> Self::Output;
        fn visit_mut_call_expr(&mut self, member: &mut CallExpr<'a>) -> Self::Output;
    }
    impl<'a> Expr<'a> {
        pub fn accept_mut<V: ExprVisitorMut<'a>>(
            &mut self,
            visitor: &mut V,
        ) -> V::Output {
            match self {
                Self::Literal(field_0) => visitor.visit_literal_expr(field_0),
                Self::Ident(field_0) => visitor.visit_ident_expr(field_0),
                Self::Binary(field_0) => visitor.visit_binary_expr(field_0),
                Self::Call(field_0) => visitor.visit_call_expr(field_0),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Expr<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Expr::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Literal",
                        &__self_0,
                    )
                }
                Expr::Ident(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Ident",
                        &__self_0,
                    )
                }
                Expr::Binary(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Binary",
                        &__self_0,
                    )
                }
                Expr::Call(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Call",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for Expr<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <Literal<
                '_,
            > as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
                || <IdentExpr<
                    '_,
                > as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
                || <BinaryExpr<
                    '_,
                > as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
                || <CallExpr<
                    '_,
                > as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
        }
    }
    impl<'a> lang::lexing::WithSpan for Expr<'a> {
        fn span(&self) -> lang::lexing::Span {
            match self {
                Expr::Literal(field_0) => field_0.span(),
                Expr::Ident(field_0) => field_0.span(),
                Expr::Binary(field_0) => field_0.span(),
                Expr::Call(field_0) => field_0.span(),
            }
        }
    }
    #[allow(unused_braces, non_snake_case)]
    pub mod expression {
        use super::*;
        use lang::{
            parsing::{Parse, Peek, TokenReader, Cursor, Error},
            lexing::{tokens::Token, WithSpan, Span},
        };
        fn __primary<'input>(
            input: &mut TokenReader<'input, '_, Token<'input>>,
        ) -> Result<Expr<'input>, Error> {
            if input.peek::<Literal>() {
                Ok(Expr::Literal(input.parse()?))
            } else {
                Ok(Expr::Ident(input.parse()?))
            }
        }
        fn __get_precedence<'input>(
            input: &mut TokenReader<'input, '_, Token<'input>>,
        ) -> u8 {
            if input.clone().parse::<BinaryOperator>().is_ok() {
                1u8
            } else if input.clone().parse::<Args>().is_ok() {
                2u8
            } else {
                0u8
            }
        }
        fn __prefix<'input>(
            input: &mut TokenReader<'input, '_, Token<'input>>,
        ) -> Result<Expr<'input>, Error> {
            if let Result::<_, Error>::Ok(ret) = __primary(input) {
                return Ok(ret)
            } else {
                Err(input.error("invalid prefix expression"))
            }
        }
        fn __infix<'input>(
            input: &mut TokenReader<'input, '_, Token<'input>>,
            left: Expr<'input>,
        ) -> Result<Expr<'input>, Error> {
            if input.clone().parse::<BinaryOperator>().is_ok() {
                let lhs = left;
                let op = input.parse::<BinaryOperator>()?;
                let rhs = __expression(input, 1u8)?;
                {
                    Ok(
                        Expr::Binary(BinaryExpr {
                            left: Box::new(lhs),
                            operator: op,
                            right: Box::new(rhs),
                        }),
                    )
                }
            } else if input.clone().parse::<Args>().is_ok() {
                let lhs = left;
                let args = input.parse::<Args>()?;
                {
                    Ok(
                        Expr::Call(CallExpr {
                            target: Box::new(lhs),
                            args: args,
                        }),
                    )
                }
            } else {
                ::core::panicking::panic_fmt(format_args!("error"))
            }
        }
        fn __expression<'input>(
            input: &mut TokenReader<'input, '_, Token<'input>>,
            precedence: u8,
        ) -> Result<Expr<'input>, Error> {
            let mut left = __prefix(input)?;
            while precedence < __get_precedence(input) {
                left = __infix(input, left)?;
            }
            Ok(left)
        }
        pub fn parser<'input>(
            input: &mut TokenReader<'input, '_, Token<'input>>,
        ) -> Result<Expr<'input>, Error> {
            __expression(input, 0)
        }
        impl<'input> Parse<'input, Token<'input>> for Expr<'input> {
            fn parse(
                input: &mut TokenReader<'input, '_, Token<'input>>,
            ) -> Result<Expr<'input>, Error> {
                parser(input)
            }
        }
    }
}
mod stmt {
    use crate::exprs::Expr;
    use lang::lexing::tokens::Ident;
    use lang::parsing::{Group, Punctuated};
    use lang::{visitor, Parse, Peek, WithSpan};
    pub type Block<'a> = Group<
        crate::tokens::OpenBrace,
        Vec<Stmt<'a>>,
        crate::tokens::CloseBrace,
    >;
    pub struct FuncStmt<'a> {
        pub fn_token: crate::tokens::Func,
        pub name: Ident<'a>,
        pub params: Group<
            crate::tokens::OpenParens,
            Punctuated<Ident<'a>, crate::tokens::Comma>,
            crate::tokens::CloseParens,
        >,
        pub body: Block<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for FuncStmt<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "FuncStmt",
                "fn_token",
                &&self.fn_token,
                "name",
                &&self.name,
                "params",
                &&self.params,
                "body",
                &&self.body,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>>
    for FuncStmt<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(FuncStmt {
                fn_token: state.parse()?,
                name: state.parse()?,
                params: state.parse()?,
                body: state.parse()?,
            })
        }
    }
    impl<'a> lang::lexing::WithSpan for FuncStmt<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.fn_token.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for FuncStmt<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <crate::tokens::Func as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub struct ForStmt<'a> {
        pub for_token: crate::tokens::For,
        pub name: Ident<'a>,
        pub in_token: crate::tokens::In,
        pub body: Block<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ForStmt<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ForStmt",
                "for_token",
                &&self.for_token,
                "name",
                &&self.name,
                "in_token",
                &&self.in_token,
                "body",
                &&self.body,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>> for ForStmt<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(ForStmt {
                for_token: state.parse()?,
                name: state.parse()?,
                in_token: state.parse()?,
                body: state.parse()?,
            })
        }
    }
    impl<'a> lang::lexing::WithSpan for ForStmt<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.for_token.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for ForStmt<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <crate::tokens::For as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub struct IfStmt<'a> {
        pub for_token: crate::tokens::If,
        pub name: Expr<'a>,
        pub body: Block<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for IfStmt<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "IfStmt",
                "for_token",
                &&self.for_token,
                "name",
                &&self.name,
                "body",
                &&self.body,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>> for IfStmt<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(IfStmt {
                for_token: state.parse()?,
                name: state.parse()?,
                body: state.parse()?,
            })
        }
    }
    impl<'a> lang::lexing::WithSpan for IfStmt<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.for_token.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for IfStmt<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <crate::tokens::If as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub struct LetStmt<'a> {
        pub let_token: crate::tokens::Let,
        pub name: Ident<'a>,
        pub assign_token: crate::tokens::Assign,
        pub value: Expr<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for LetStmt<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "LetStmt",
                "let_token",
                &&self.let_token,
                "name",
                &&self.name,
                "assign_token",
                &&self.assign_token,
                "value",
                &&self.value,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>> for LetStmt<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(LetStmt {
                let_token: state.parse()?,
                name: state.parse()?,
                assign_token: state.parse()?,
                value: state.parse()?,
            })
        }
    }
    impl<'a> lang::lexing::WithSpan for LetStmt<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.let_token.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for LetStmt<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <crate::tokens::Let as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub struct ReturnStmt<'a> {
        pub return_token: crate::tokens::Return,
        pub expr: Expr<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ReturnStmt<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ReturnStmt",
                "return_token",
                &&self.return_token,
                "expr",
                &&self.expr,
            )
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>>
    for ReturnStmt<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            Ok(ReturnStmt {
                return_token: state.parse()?,
                expr: state.parse()?,
            })
        }
    }
    impl<'a> lang::lexing::WithSpan for ReturnStmt<'a> {
        fn span(&self) -> lang::lexing::Span {
            self.return_token.span()
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for ReturnStmt<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <crate::tokens::Return as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
        }
    }
    pub enum Stmt<'a> {
        Func(FuncStmt<'a>),
        Let(LetStmt<'a>),
        If(IfStmt<'a>),
        Return(ReturnStmt<'a>),
    }
    pub trait StmtVisitor<'a> {
        type Output;
        fn visit_func_stmt(&mut self, member: &FuncStmt<'a>) -> Self::Output;
        fn visit_let_stmt(&mut self, member: &LetStmt<'a>) -> Self::Output;
        fn visit_if_stmt(&mut self, member: &IfStmt<'a>) -> Self::Output;
        fn visit_return_stmt(&mut self, member: &ReturnStmt<'a>) -> Self::Output;
    }
    impl<'a> Stmt<'a> {
        pub fn accept<V: StmtVisitor<'a>>(&self, visitor: &mut V) -> V::Output {
            match self {
                Self::Func(field_0) => visitor.visit_func_stmt(field_0),
                Self::Let(field_0) => visitor.visit_let_stmt(field_0),
                Self::If(field_0) => visitor.visit_if_stmt(field_0),
                Self::Return(field_0) => visitor.visit_return_stmt(field_0),
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Stmt<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Stmt::Func(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Func",
                        &__self_0,
                    )
                }
                Stmt::Let(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Let",
                        &__self_0,
                    )
                }
                Stmt::If(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "If", &__self_0)
                }
                Stmt::Return(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Return",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl<
        'parse: 'a,
        'a,
    > lang::parsing::Parse<'parse, lang::lexing::tokens::Token<'parse>> for Stmt<'a> {
        fn parse(
            state: &mut lang::parsing::TokenReader<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> Result<Self, lang::parsing::Error> {
            if state.peek::<FuncStmt<'_>>() {
                return Ok(Stmt::Func(state.parse()?))
            } else if state.peek::<LetStmt<'_>>() {
                return Ok(Stmt::Let(state.parse()?))
            } else if state.peek::<IfStmt<'_>>() {
                return Ok(Stmt::If(state.parse()?))
            } else if state.peek::<ReturnStmt<'_>>() {
                return Ok(Stmt::Return(state.parse()?))
            }
            Err(
                state
                    .error((
                        "Stmt",
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                lang::parsing::ErrorKind::Expected {
                                    message: "Func".into(),
                                    rule: Some("Func".into()),
                                },
                                lang::parsing::ErrorKind::Expected {
                                    message: "Let".into(),
                                    rule: Some("Let".into()),
                                },
                                lang::parsing::ErrorKind::Expected {
                                    message: "If".into(),
                                    rule: Some("If".into()),
                                },
                                lang::parsing::ErrorKind::Expected {
                                    message: "Return".into(),
                                    rule: Some("Return".into()),
                                },
                            ]),
                        ),
                    )),
            )
        }
    }
    impl<'a> lang::lexing::WithSpan for Stmt<'a> {
        fn span(&self) -> lang::lexing::Span {
            match self {
                Stmt::Func(field_0) => field_0.span(),
                Stmt::Let(field_0) => field_0.span(),
                Stmt::If(field_0) => field_0.span(),
                Stmt::Return(field_0) => field_0.span(),
            }
        }
    }
    impl<'parse: 'a, 'a> lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>
    for Stmt<'a> {
        fn peek(
            cursor: &mut lang::parsing::Cursor<
                'parse,
                '_,
                lang::lexing::tokens::Token<'parse>,
            >,
        ) -> bool {
            use lang::parsing::Peek;
            <FuncStmt<
                '_,
            > as lang::parsing::Peek<
                'parse,
                lang::lexing::tokens::Token<'parse>,
            >>::peek(cursor)
                || <LetStmt<
                    '_,
                > as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
                || <IfStmt<
                    '_,
                > as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
                || <ReturnStmt<
                    '_,
                > as lang::parsing::Peek<
                    'parse,
                    lang::lexing::tokens::Token<'parse>,
                >>::peek(cursor)
        }
    }
}
fn main() {
    let input = "\n\nfn fib(n) {\n    if n <= 1 {\n        return 1\n    }\n\n\n    return fib(n - 1) + fib(n - 2)\n}";
    let lexer = tokens::Lexer::new(input);
    let mut parser = Parser::from_tokens(input, lexer.skip_whitespace(true).tokenize())
        .expect("lex");
    let stmts = parser.parse::<Stmt>().expect("message");
    {
        ::std::io::_print(format_args!("{0:#?}\n", stmts));
    };
}
