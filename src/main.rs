use std::env;
mod machine;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm = machine::Chip8Machine::new();
    match vm.load_program(&args[1]) {
        Ok(_) => vm.run(),
        Err(e) => panic!("{:?}", e),
    }
}
