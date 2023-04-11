use lalrpop_util::lalrpop_mod;

lalrpop_mod!(game8);
use game8::ProgParser;

fn main() {
    let prog = "
        let n = 10;
        let sum = 0;

        while n > 0 do
            let sum = sum + n;
            let n = n - 1;
        endwhile
        ";
    println!("{:?}", ProgParser::new().parse(prog));
}
