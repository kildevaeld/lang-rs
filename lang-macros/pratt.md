

lang::precedence! {
    expression => Expr<'input>
    lhs:@ op:( "=" !"=" { BinaryOp::Assign }) rhs:@ {}
    --
    lha:@ op:( "*" )
    --
    binary op:"*" {  }
    --
    b:primary { b }
}