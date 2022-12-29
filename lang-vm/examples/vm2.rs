use lang_vm::{
    bytecode::builder2::*,
    error,
    error::Result,
    gc::{Collect, MutationContext},
    Callable, Closure, NativeFunc, StringInterner, Value, Vm,
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
    let interner = StringInterner::new(mc);

    let mut module = Module::new(mc, interner);

    let fib = module.create_function(mc, "fib");

    let this = module.resolve_local("fib").expect("this");

    let num = fib.define_local(mc, "num");
    let one = fib.constant(mc, Value::Integer(1));
    let two = fib.constant(mc, Value::Integer(2));

    let start = fib.create_section(mc);
    let els = fib.create_section(mc);

    start
        .push(mc, expr::jump_if_false(expr::lte(num, one), els))
        .push(mc, expr::pop())
        .push(mc, expr::returns(one));

    els.push(mc, Code::Pop)
        .push(mc, this)
        .push(mc, expr::sub(num, one))
        .push(mc, Code::Call(1))
        .push(mc, this)
        .push(mc, expr::sub(num, two))
        .push(mc, Code::Call(1))
        .push(mc, BinaryOperator::Add)
        .push(mc, Code::Return);

    println!("ctx: {}", module.build(mc).chunk());

    todo!()
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
