use lang::{
    parsing::{Group, Punctuated},
    visitor, Parse, Peek, WithSpan,
};
use lang_lexing::tokens::{Ident, Literal};

#[derive(Debug, Parse, WithSpan, Peek)]
pub enum BinaryOperator {
    Add(Token![+]),
    Sub(Token![-]),
    Lte(Token![<=]),
}

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct IdentExpr<'a> {
    pub ident: Ident<'a>,
}

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct BinaryExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: BinaryOperator,
    pub right: Box<Expr<'a>>,
}

pub type Args<'a> = Group<Token!["("], Punctuated<Expr<'a>, Token![,]>, Token![")"]>;

#[derive(Debug, Parse, WithSpan, Peek)]
pub struct CallExpr<'a> {
    pub target: Box<Expr<'a>>,
    pub args: Args<'a>,
}

#[derive(Debug, WithSpan)]
#[visitor(with_mut = true)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    Ident(IdentExpr<'a>),
    Binary(BinaryExpr<'a>),
    Call(CallExpr<'a>),
}

lang::precedence! {
    expression -> Expr<'input>
    rule lhs:@ op:BinaryOperator rhs:@ {
        Ok(Expr::Binary(BinaryExpr {
            left: Box::new(lhs),
            operator:op,
            right: Box::new(rhs)
        }))
    }
    --
    rule lhs:@ args:Args {
        Ok(Expr::Call(CallExpr {
            target: Box::new(lhs),
            args: args
        }))
    }
    --
    rule o:Literal {
        Ok(Expr::Literal(o))
    } / i:Ident {
        Ok(Expr::Ident(IdentExpr {
            ident: i
        }))
    }
}
