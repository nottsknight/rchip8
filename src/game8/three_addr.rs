use super::ast::Statement;

struct TAC<'a>(&'a str, TACInst);

enum TACInst {
    Val,
}

fn stmt_to_tac<'a>(stmt: &'a Statement) -> TAC<'a> {
    match stmt {
        Statement::Assign(var, val) => TAC(var, TACInst::Val),
        _ => panic!(),
    }
}
