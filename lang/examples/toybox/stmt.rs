use crate::exprs::Expr;
use lang::lexing::tokens::Ident;
use lang::parsing::{Group, Punctuated};
use lang::{visitor, Parse, Peek, WithSpan};

pub type Block<'a> = Group<Token!["{"], Vec<Stmt<'a>>, Token!["}"]>;

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct FuncStmt<'a> {
    pub fn_token: Token![fn],
    pub name: Ident<'a>,
    pub params: Group<Token!["("], Punctuated<Ident<'a>, Token![,]>, Token![")"]>,
    pub body: Block<'a>,
}

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct ForStmt<'a> {
    pub for_token: Token![for],
    pub name: Ident<'a>,
    pub in_token: Token![in],
    pub body: Block<'a>,
}

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct IfStmt<'a> {
    pub for_token: Token![if],
    pub name: Expr<'a>,
    pub body: Block<'a>,
}

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct LetStmt<'a> {
    pub let_token: Token![let],
    pub name: Ident<'a>,
    pub assign_token: Token![=],
    pub value: Expr<'a>,
}

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct ReturnStmt<'a> {
    pub return_token: Token![return],
    pub expr: Expr<'a>,
}

#[derive(Debug, Parse, WithSpan, Peek)]
#[visitor]
pub enum Stmt<'a> {
    Func(FuncStmt<'a>),
    Let(LetStmt<'a>),
    If(IfStmt<'a>),
    Return(ReturnStmt<'a>),
}
