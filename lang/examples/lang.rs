use lang::{
    lexing::tokens::{Ident, Literal, LiteralNumber, LiteralString, Punct, Token},
    parsing::{Error, Ident as PeekIdent, Parse, Parser, Punctuated, TokenReader},
};

#[derive(Debug)]
pub enum Expr {
    Add { left: Box<Expr>, right: Box<Expr> },
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
    Div { left: Box<Expr>, right: Box<Expr> },
    Ident(String),
    Literal(String),
    Call { object: Box<Expr>, params: Params },
}

#[derive(Debug)]
pub struct Params {
    exprs: Vec<Expr>,
}

impl<'a> Parse<'a, Token<'a>> for Params {
    fn parse(state: &mut TokenReader<'a, '_, Token<'a>>) -> Result<Self, Error> {
        let parser = Punctuated::<Expr, Token![,]>::terminated;

        let exprs = parser(state)?.into_iter().collect();

        Ok(Params { exprs })
    }
}

pub mod tokens {
    use super::*;
    lang::tokens!(
        Token
        module_path: tokens
        puncts {
            "+" Add,
            "-" Sub,
            "*" Mul,
            "/" Div,
            "(" OpenParens,
            ")" CloseParens,
            "{" OpenBrace,
            "}" CloseBrace,
            "," Comma,

        }
        keywords {
            "fn" Func
        }
        literal { LiteralString, LiteralNumber }
    );
}

lang::precedence!(
    expression -> Expr
    rule lhs:@ op:([+] [-]) rhs:@ {

        let i = op.slice(input.input()).unwrap();

        if i == "+" {
            Ok(Expr::Add {
                left: Box::new(lhs),
                right: Box::new(rhs)
            })
        } else {
            Ok(Expr::Sub {
                left: Box::new(lhs),
                right: Box::new(rhs)
            })
        }

    }
    --
    rule lhs:@ op:([/] [*]) rhs:@ {
        let i = op.slice(input.input()).unwrap();

        if i == "/" {
            Ok(Expr::Div {
                left: Box::new(lhs),
                right: Box::new(rhs)
            })
        } else {
            Ok(Expr::Mul {
                left: Box::new(lhs),
                right: Box::new(rhs)
            })
        }
    }
    --
    rule "(" e:@ ")" { Ok(e) }
    --
    rule lhs:@ "(" params:Params ")" { Ok(Expr::Call {
        object: Box::new(lhs),
        params
    }) }
    --
    {
        if input.peek(PeekIdent) {
            Ok(Expr::Ident(input.parse::<Ident>()?.lexeme.to_string()))
        } else {
            Ok(Expr::Literal(input.parse::<Literal>()?.lexeme.to_string()))
        }


    }
);

fn main() {
    let lexer = tokens::Lexer::new("fn test() { 2+4 * 5 / test - call((2 + 4) * 2) }")
        .skip_whitespace(true);

    let mut parser = Parser::from_tokens(lexer.input(), lexer.tokenize()).expect("tokenize");

    let out = parser.parse::<(
        Token![fn],
        Ident,
        Token!["("],
        Token![")"],
        Token!["{"],
        Expr,
        Token!["}"],
    )>();

    println!("{:#?}", out);
}
