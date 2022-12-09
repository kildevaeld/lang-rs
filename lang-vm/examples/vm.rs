use gc_arena::{Collect, Gc, MutationContext};
use lang_vm::{
    bytecode::{
        builder::{self, expr, BinaryOperator, Code, Top},
        Chunk, Function, Opcode,
    },
    error,
    error::Result,
    Callable, Closure, NativeFunc, String, Value, Vm,
};

#[derive(Debug, Collect)]
#[collect(no_drop)]
pub struct Test;

impl<'gc> NativeFunc<'gc> for Test {
    fn call(&self, args: &[Value<'gc>]) -> error::Result<Value<'gc>> {
        Ok(args[0])
    }
}

fn create_fib<'gc, 'a>(mc: MutationContext<'gc, 'a>) -> Closure<'gc> {
    let mut func = builder::Function::new(mc);

    let num = func.define_arg();
    let one = func.constant(Value::Integer(1));
    let two = func.constant(Value::Integer(2));

    let this = func.this();

    let els = func.create_section();

    func.push(expr::jump_if_false(expr::lte(num, one), els))
        .push(expr::pop())
        .push(expr::returns(one));

    func[els]
        .push(Code::Pop)
        .push(this)
        .push(expr::sub(num, one))
        .push(Code::Call(1))
        .push(this)
        .push(expr::sub(num, two))
        .push(Code::Call(1))
        .push(BinaryOperator::Add)
        .push(Code::Return);

    let func = func.build();

    //println!("{}", func.chunk());

    Closure::new(mc, func, 0)
}

fn main() {
    let mut vm = Vm::new();

    vm.mutate(|ctx| {
        let fib = create_fib(*ctx);

        /*let ops = vec![
            Opcode::GetLocal as u8,
            1,
            Opcode::Constant as u8,
            0,
            Opcode::Call1 as u8,
            Opcode::Return as u8,
        ];

        let consts = vec![Value::String(String::new_static("Hello, World!"))];

        let chunk = Closure::new(*ctx, Function::new(Chunk::new(ops, consts), 1), 0);

        // println!("{}", chunk.chunk());

        let ret = ctx
            .call(
                Callable::Closure(chunk),
                &[Value::Callable(Callable::Native(Gc::allocate(
                    ctx.mc,
                    Box::new(Test),
                )))],
            )
            .unwrap();*/

        // println!("Ret {}", ret.as_string().unwrap());

        let ret = ctx
            .call(Callable::Closure(fib), &[Value::Integer(20)])
            .unwrap();

        println!("value {:?}", ret);

        Result::Ok(())
    })
    .expect("vm");

    println!("allocated: {}", vm.total_allocated());

    // std::thread::sleep(std::time::Duration::from_secs(60));
}
