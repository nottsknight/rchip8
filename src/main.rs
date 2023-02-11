use rchip8::machine::VirtualMachine;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm = VirtualMachine::new();
    match vm.load_program(&args[1]) {
        Ok(_) => vm.run(),
        Err(e) => panic!("{:?}", e),
    }
}
