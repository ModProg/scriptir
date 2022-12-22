use budvm::VirtualMachine;
use scriptir::ir::*;

#[test]
fn execute() {
    let script: Script = serde_yaml::from_str(include_str!("ir/test.yml")).unwrap();

    let mut vm = VirtualMachine::empty();

    let script = script.compile(&mut vm).unwrap();

    let smth: f64 = vm.run(script.code.into(), script.variables).unwrap();
    assert_eq!(smth, 10.);
}
