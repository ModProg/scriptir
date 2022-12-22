use budvm::{VirtualMachine, Instruction};

fn main() {
    let mut context = VirtualMachine::empty();
    let result: i64 = context.run(std::borrow::Cow::Borrowed(&[
            Instruction::Push(budvm::ValueOrSource::Value(budvm::Value::Integer(10)))
    ]), 0).unwrap();
    dbg!(result);
}
