use lalrpop_util::lalrpop_mod;
use std::io::{BufRead, Write};
use symbolrust::prelude::*;

lalrpop_mod!(grammar);

#[derive(Debug)]
enum UserInput {
    Assignment(Variable, Node),
    Expr(Node),
}

fn main() {
    let mut context = Context::new();

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
            .parse(&mut context, &line)
            .unwrap_or_else(|e| panic!("{}", e));

        match user_input {
            UserInput::Assignment(v, e) => {
                let evaluated = Evaluator::evaluate(&e, &context);
                let folded = ConstantFolder::fold(&evaluated);
                let as_str = context.print(&folded);
                // Cannot panic because we register the name during parsing.
                let name = context.get_name(v).unwrap();
                println!("\t{} = {}", name, as_str);
                context.assign(v, e);
            }
            UserInput::Expr(e) => {
                let evaluated = Evaluator::evaluate(&e, &context);
                let folded = ConstantFolder::fold(&evaluated);
                let as_str = context.print(&folded);
                println!("\t{}", as_str);
            }
        }
        new_line();
    }
}
