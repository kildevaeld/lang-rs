use lang_lexing::tokens::{Ident, Literal, LiteralNumber};
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
            "==" Equal,
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
    Mul,
    Assign,
    Eq,
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
}

lang::precedence3! {
    expression -> Expr<'input>
    rule lhs:@ "=" !"=" rhs:@ {
        Ok(Expr::Binary {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op: BinaryOperator::Assign
        })
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
    rule o:Literal {
        Ok(Expr::Lit(o))
    } / i:Ident {
        Ok(Expr::Ident(i))
    }

}

fn main() {
    let input = "ident = 20 - 100 / 2 == 202";

    let lexer = tokens::Lexer::new(input);

    let mut parser =
        Parser::from_tokens(input, lexer.skip_whitespace(true).tokenize()).expect("lex");

    let ast = parser.parse::<Expr>().expect("message");

    println!("{:#?}", ast)
}
