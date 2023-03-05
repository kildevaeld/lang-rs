use lang::{
    lexing::tokens::{Ident, Literal, LiteralNumber, LiteralString, Punct, Token},
    parsing::{Error, Parse, Parser, Punctuated, TokenReader},
    Peek, WithSpan,
};

#[derive(Debug, Peek)]
pub enum Expr {
    Add { left: Box<Expr>, right: Box<Expr> },
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
    Div { left: Box<Expr>, right: Box<Expr> },
    Ident(String),
    Literal(String),
    Call { object: Box<Expr>, params: Params },
}

#[derive(Debug, Peek)]
pub struct Params {
    exprs: Vec<Expr>,
}

impl<'a> Parse<'a, Token<'a>> for Params {
    fn parse(state: &mut TokenReader<'a, '_, Token<'a>>) -> Result<Self, Error> {
        let exprs = Punctuated::<Expr, Token![,]>::parse(state)?;

        // let exprs = parser(state)?.into_iter().collect();

        Ok(Params { exprs })
    }
}

pub mod tokens {
    use super::*;

    lang::tokens!(
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
            "=" Assign

        }
        keywords {
            "fn" Func,
            "let" Let
        }
        literals { LiteralString, LiteralNumber }
    );
}

lang::precedence!(
    expression -> Expr
    lhs:@ op:([+] [-]) rhs:@ {

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
    lhs:@ op:([/] [*]) rhs:@ {
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
    "(" e:@ ")" { Ok(e) }
    --
    lhs:@ "(" params:Params ")" { Ok(Expr::Call {
        object: Box::new(lhs),
        params
    }) }
    --
    {
        if input.peek::<Ident>() {
            Ok(Expr::Ident(input.parse::<Ident>()?.lexeme.to_string()))
        } else {
            Ok(Expr::Literal(input.parse::<Literal>()?.lexeme.to_string()))
        }


    }
);

fn main() {
    let lexer = tokens::Lexer::new(
        r#"
    let test = 200;
    "#,
    )
    .skip_whitespace(true);

    let mut parser = Parser::from_tokens(lexer.input(), lexer.tokenize()).expect("tokenize");

    let out = parser.parse::<(Token![let], Ident, Token![=], Expr)>();

    println!("{:#?}", out);
}
