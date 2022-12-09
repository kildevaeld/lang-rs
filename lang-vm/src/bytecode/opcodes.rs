lang_macros::opcodes!(
    Pop(take: 1),
    Constant(constant),
    GetLocal(local, out: 1),
    SetLocal(local, take: 1),
    GetGlobal(constant),
    Closure(constant),
    JumpIfFalse(take: 1, offset),
    Jump(offset),
    Add(take: 2, out: 1),
    Sub(take: 2, out: 1),
    Mul(take: 2, out: 1),
    Div(take: 2, out: 1),
    Not(take: 1, out: 1),
    Gt(take: 2, out: 1),
    Lt(take: 2, out: 1),
    Call0(take: 1, out: 1),
    Call1(take: 2, out: 1),
    Call2(take: 3, out: 1),
    Call3(take: 4, out: 1),
    Return(take: 1),
    Export(take: 2),
);
