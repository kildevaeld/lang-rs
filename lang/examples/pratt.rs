use lang_lexing::tokens::{Literal, LiteralNumber};
use lang_parsing::Parser;

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
            "(" OpenParens,
            ")" CloseParens,
            "{" OpenBrace,
            "}" CloseBrace,
            "," Comma,
            "=" Assign

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
    Assign,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Lit(Literal<'a>),
    Binary {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
        op: BinaryOperator,
    },
}

lang::precedence2! {
    expression -> Expr<'input>
    lhs:@ "=" !"=" rhs:@ {
        Ok(Expr::Binary {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op: BinaryOperator::Add
        })
    }
    --
    lhs:@ op:(("+" { BinaryOperator::Add }) / ("-" { BinaryOperator::Sub })) rhs:@ {
        Ok(Expr::Binary {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op
        })
    }
    // --
    o:Literal {
        Ok(Expr::Lit(o))
    }
}

fn main() {
    let input = "20 - 100 / 2";

    let lexer = tokens::Lexer::new(input);

    let mut parser =
        Parser::from_tokens(input, lexer.skip_whitespace(true).tokenize()).expect("lex");
}
