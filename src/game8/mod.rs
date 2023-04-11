pub mod ast;
use ast::{FuncDefinition, Program, Statement};
use std::collections::HashSet;

pub fn collect_func_names(Program(fs, _): &Program) -> HashSet<String> {
    let mut names = HashSet::new();
    for FuncDefinition(name, _, _) in fs {
        names.insert(name.clone());
    }
    names
}

pub fn collect_var_names(Program(fs, stmts): &Program) -> HashSet<String> {
    let mut vars = HashSet::new();

    for FuncDefinition(name, params, body) in fs {
        for p in params {
            vars.insert(format!("{}__{}", name, p));
        }

        for s in body {
            if let Statement::Assign(v, _) = s.as_ref() {
                vars.insert(format!("{}__{}", name, v));
            }
        }
    }

    for s in stmts {
        if let Statement::Assign(v, _) = s.as_ref() {
            vars.insert(v.clone());
        }
    }

    vars
}
