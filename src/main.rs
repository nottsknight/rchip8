use std::process::exit;

use rchip8::interpreter;

fn main() {
    let mut interpreter = interpreter::Chip8Interpreter::new();
    match interpreter.load_program("test.c8") {
        Ok(_) => match interpreter.run() {
            Ok(_) => (),
            Err(e) => {
                println!("Execution failed: {}", e);
                exit(2);
            }
        },
        Err(e) => {
            println!("Failed to run program: {}", e);
            exit(1);
        }
    }
}
