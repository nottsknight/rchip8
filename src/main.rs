use rchip8::machine::VirtualMachine;

fn main() {
    let mut vm = VirtualMachine::new();
    match vm.load_program("roms/ibm-logo.ch8") {
        Ok(_) => vm.run(),
        Err(e) => panic!("{:?}", e),
    }
}
