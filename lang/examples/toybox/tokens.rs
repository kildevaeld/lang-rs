use lang::lexing::tokens::{LiteralNumber, LiteralString};

lang::tokens!(
    module_path: tokens
    puncts {
        "+" Add,
        "-" Sub,
        "*" Mul,
        "/" Div,
        "<=" Lte,
        "(" OpenParens,
        ")" CloseParens,
        "{" OpenBrace,
        "}" CloseBrace,
        "," Comma,
        "=" Assign,
        ";" Semi

    }
    keywords {
        "fn" Func,
        "let" Let,
        "for" For,
        "in" In,
        "if" If,
        "else" Else,
        "return" Return
    }
    literal { LiteralString, LiteralNumber }
);
