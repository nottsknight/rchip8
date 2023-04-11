pub mod ast;
use ast::{Expression, Program, Statement};

use self::ast::FuncDefinition;

fn is_builtin(name: &str) -> bool {
    match name {
        "read_delay" | "set_delay" | "set_sound" | "input" => true,
        _ => false,
    }
}

fn is_invalid_func(name: &str, def_names: &Vec<String>) -> bool {
    is_builtin(name) || !def_names.contains(&String::from(name))
}

fn check_builtins(Program(defs, stmts): Program) -> bool {
    let mut func_names = Vec::new();
    for FuncDefinition(name, _, _) in defs {
        if is_builtin(&name) {
            return false;
        }
        func_names.push(name.clone());
    }

    for s in stmts {
        match *s {
            Statement::Assign(_, e) => {
                if let Expression::FuncCallExpr(name, _) = *e {
                    if is_invalid_func(&name, &func_names) {
                        return false;
                    }
                }
            }
            Statement::FuncCallStmt(name, _) => {
                if is_invalid_func(&name, &func_names) {
                    return false;
                }
            }
            _ => (),
        }
    }
    true
}
