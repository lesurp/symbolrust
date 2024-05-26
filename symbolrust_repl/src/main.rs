use lalrpop_util::lalrpop_mod;
use std::collections::{hash_map::Entry, HashMap};
use std::io::{BufRead, Write};
use symbolrust::prelude::*;
use symbolrust::visitors::PrettyPrinterContext;

lalrpop_mod!(grammar);

#[derive(Debug)]
enum UserInput {
    Assignment(Variable, Node),
    Expr(Node),
}

fn main() {
    let mut var_map = VariableMap::new();
    let mut var_context = VariableContext::new();

    let stdin = std::io::stdin();

    let new_line = || {
        print!(">> ");
        std::io::stdout().flush().unwrap();
    };

    new_line();
    for line in stdin.lock().lines() {
        let line = if let Ok(line) = line {
            line
        } else {
            break;
        };

        let user_input = grammar::LineParser::new()
            .parse(&mut var_map, &line)
            .unwrap_or_else(|e| panic!("{}", e));

        match user_input {
            UserInput::Assignment(v, e) => {
                let evaluated = Evaluator::evaluate(&e, &var_context);
                let folded = ConstantFolder::fold(&evaluated);
                let as_str = PrettyPrinter::print_with_context(&folded, var_map.as_context());
                let name = var_map.var(v).unwrap();
                println!("\t{} = {}", name, as_str);
                var_context.insert(v, e);
            }
            UserInput::Expr(e) => {
                let evaluated = Evaluator::evaluate(&e, &var_context);
                let folded = ConstantFolder::fold(&evaluated);
                let as_str = PrettyPrinter::print_with_context(&folded, var_map.as_context());
                println!("\t{}", as_str);
            }
        }
        new_line();
    }
}
