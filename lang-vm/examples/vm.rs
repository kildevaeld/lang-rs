use lang_vm::{
    bytecode::builder::{self, expr, BinaryOperator, Code},
    error,
    error::Result,
    gc::{Collect, MutationContext},
    Callable, Closure, NativeFunc, Value, Vm,
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

    Closure::new(mc, func, 0)
}

fn main() {
    let mut vm = Vm::new();

    vm.mutate(|ctx| {
        let fib = create_fib(*ctx);

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
