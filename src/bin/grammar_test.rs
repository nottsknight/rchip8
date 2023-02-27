#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

fn main() {
    let test_input = "gcd_function";
    let test_output = grammar::ExpressionParser::new().parse(test_input);
    println!("{:?}", test_output);
}
