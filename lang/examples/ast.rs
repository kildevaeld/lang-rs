lang::ast!(
    Expr<'a> {
        Binary<'a> {
            left: Box<Expr<'a>>,
            right: Box<Expr<'a>>,
        }
    }
);

fn main() {}
