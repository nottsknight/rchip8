use lalrpop_util::lalrpop_mod;

lalrpop_mod!(game8);
use game8::ProgParser;

fn main() {
    let prog = "
        let n = input();
        let sum = 0;

        while n > 0 do
            let sum = sum + n;
            let n = n - 1;
        endwhile
        draw_number(sum);
        ";
    println!("{:?}", ProgParser::new().parse(prog));
}
