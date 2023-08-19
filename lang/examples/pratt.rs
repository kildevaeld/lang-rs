use lang::{Parse, Peek, WithSpan};
use lang_lexing::tokens::{Ident, Literal, LiteralNumber};
use lang_parsing::{tokens::Group, tokens::NoWs, Parser};

#[macro_use]
pub mod tokens {
    use super::*;
    lang::tokens!(
        module_path: tokens
        puncts {
            "+" Add,
            "-" Sub,
            "*" Mul,
            "/" Div,
            "==" Equal,
            "(" OpenParens,
            ")" CloseParens,
            "{" OpenBrace,
            "}" CloseBrace,
            "," Comma,
            "=" Assign,
            ":" Colon,
            "?" Question

        }
        keywords {
            "fn" Func
        }
        literal { LiteralNumber }
    );
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Sub,
    Add,
    Div,
    Mul,
    Assign,
    Eq,
}

#[derive(Debug, Clone)]
pub struct Tenary<'a> {
    pub expr: Box<Expr<'a>>,
    pub cons: Box<Expr<'a>>,
    pub alt: Box<Expr<'a>>,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Lit(Literal<'a>),
    Binary {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
        op: BinaryOperator,
    },
    Ident(Ident<'a>),
    Tenary(Tenary<'a>),
}

lang::precedence! {
    expression -> Expr<'input>
    rule lhs:@ "=" !"=" rhs:@ {
        Ok(Expr::Binary {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op: BinaryOperator::Assign
        })
    }
    --
    rule expr:@ "?" cons:@ ":" alt:@ {
        Ok(Expr::Tenary(Tenary {
            expr: Box::new(expr),
            cons: Box::new(cons),
            alt: Box::new(alt)
        }))
    }
    --
    rule lhs:@ "==" rhs:@ {
        Ok(Expr::Binary {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op: BinaryOperator::Eq
        })
    }
    --
    rule lhs:@ op:("+" { BinaryOperator::Add } / "-" { BinaryOperator::Sub }) rhs:@ {
        Ok(Expr::Binary {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op
        })
    }
    --
    rule lhs:@ op:("/" { BinaryOperator::Div } / "*" { BinaryOperator::Mul }) rhs:@ {
        Ok(Expr::Binary {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op
        })
    }

    --
    rule "(" e:@ ")" {
        Ok(e)
    }
    rule o:Literal {
        Ok(Expr::Lit(o))
    } / i:Ident {
        Ok(Expr::Ident(i))
    }

}

#[derive(Debug, Parse, Peek, WithSpan)]
struct Fn<'a> {
    fn_token: Token![fn],
    name: Ident<'a>,
    params: (Token!["("], Token![")"]),
    body: Group<Token!["{"], Vec<Expr<'a>>, Token!["}"]>,
}

fn main() {
    let input = "ident = 20 - 100 / 2 == 202 ? true : false";

    let input = r#"
    fn test() {
        200 +test mig / 20202
    }
    "#;

    let lexer = tokens::Lexer::new(input);

    let mut parser =
        Parser::from_tokens(input, lexer.skip_whitespace(false).tokenize()).expect("lex");

    let ast = parser.parse::<Fn>().expect("message");

    println!("{:#?}", ast)
}
