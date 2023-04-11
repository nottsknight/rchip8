use lalrpop_util::lalrpop_mod;

lalrpop_mod!(game8);
use game8::ProgParser;

fn main() {
    let prog = "
        def foo(x, y) do
            return x + y;
        enddef

        let a = foo(1, 2);
        ";
    println!("{:?}", ProgParser::new().parse(prog));
}
