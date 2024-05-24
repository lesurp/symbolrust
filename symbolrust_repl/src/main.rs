// required because of pest generated code........
#![allow(clippy::upper_case_acronyms)]

use lazy_static::lazy_static;
use pest::{
    iterators::Pair,
    iterators::Pairs,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};
use pest_derive::Parser;
use std::collections::{hash_map::Entry, HashMap};
use std::io::{BufRead, Write};
use symbolrust::prelude::*;
use symbolrust::visitors::PrettyPrinterContext;

// TODO: pest sucks pretty bad, ditch it (custom parser or another lib)
// nom seems overkill, but mb?
#[derive(Parser)]
#[grammar = "../grammar.pest"] // relative to src
struct MyParser;

#[derive(Default)]
struct VariableMap {
    v2n: PrettyPrinterContext,
    n2v: HashMap<String, Variable>,
}

impl VariableMap {
    fn new() -> Self {
        Self::default()
    }

    pub fn name(&mut self, n: String) -> Variable {
        match self.n2v.entry(n) {
            Entry::Vacant(vac) => {
                let var = Variable::new();
                self.v2n.name_var(var, vac.key().clone());
                *vac.insert(var)
            }
            Entry::Occupied(occ) => *occ.get(),
        }
    }

    pub fn var(&mut self, v: Variable) -> Option<&String> {
        self.v2n.as_map().get(&v)
    }

    pub fn as_context(&self) -> &PrettyPrinterContext {
        &self.v2n
    }
}

struct AstBuilder {
    fn_to_ctor: HashMap<String, fn(Vec<Node>) -> FunctionResult>,
}

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
            Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
            Operator::new(Rule::power, Assoc::Right),
        ])
    };
}

fn infix(lhs: Node, op: Pair<Rule>, rhs: Node) -> Node {
    match op.as_rule() {
        Rule::add => lhs + rhs,
        Rule::subtract => lhs - rhs,
        Rule::multiply => lhs * rhs,
        Rule::divide => lhs / rhs,
        Rule::power => lhs ^ rhs,
        _ => unreachable!(),
    }
}

enum TopLevelExpression {
    Expr(Node),
    Assignment(Variable, Node),
}

impl AstBuilder {
    fn new() -> Self {
        let mut fn_to_ctor = HashMap::new();
        fn_to_ctor.insert("exp".to_owned(), Exponential::from_args as _);
        AstBuilder { fn_to_ctor }
    }

    pub fn build_top_level<'i>(
        &mut self,
        mut pair: Pairs<'i, Rule>,
        var_map: &mut VariableMap,
    ) -> TopLevelExpression {
        let pair = pair.next().unwrap();
        match pair.as_rule() {
            Rule::assignment => {
                let mut it = pair.into_inner();
                let var_name = it.next().unwrap().as_str();
                let expr = it.next().unwrap();
                let node = self.build_expr(expr.into_inner(), var_map);
                let var = var_map.name(var_name.to_owned());

                TopLevelExpression::Assignment(var, node)
            }
            Rule::expr => TopLevelExpression::Expr(self.build_expr(pair.into_inner(), var_map)),
            _ => unreachable!(),
        }
    }

    fn build_expr<'i>(&mut self, pair: Pairs<'i, Rule>, var_map: &mut VariableMap) -> Node {
        PREC_CLIMBER.climb(pair, |p| self.build_subexpr(p, var_map), infix)
    }

    fn build_subexpr<'i>(&mut self, pair: Pair<'i, Rule>, var_map: &mut VariableMap) -> Node {
        match pair.as_rule() {
            Rule::expr => self.build_expr(pair.into_inner(), var_map),
            Rule::constant => pair.as_str().parse::<f64>().unwrap().into(),
            Rule::var => {
                let var_name = pair.as_str().to_owned();
                var_map.name(var_name).into()
            }
            Rule::function_call => {
                let mut it = pair.into_inner();
                let fn_name = it.next().unwrap().as_str();
                let args = it
                    .map(|pair| self.build_expr(pair.into_inner(), var_map))
                    .collect();
                if let Some(func) = self.fn_to_ctor.get(fn_name) {
                    func(args).unwrap()
                } else {
                    panic!("Unknown function");
                }
            }

            // shouldn't appear at all
            Rule::operation
            | Rule::identifier
            | Rule::line
            | Rule::term
            | Rule::int
            | Rule::num
            | Rule::WHITESPACE
            | Rule::assignment => unreachable!(),
            // consumed by the prec_climber
            Rule::add | Rule::subtract | Rule::multiply | Rule::divide | Rule::power => {
                unreachable!();
            }
        }
    }
}

fn main() {
    let mut ast_builder = AstBuilder::new();

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

        let pairs = MyParser::parse(Rule::line, &line).unwrap_or_else(|e| panic!("{}", e));
        let result = ast_builder.build_top_level(pairs, &mut var_map);

        match result {
            TopLevelExpression::Expr(expr) => {
                let evaluated = Evaluator::evaluate(&expr, &var_context);
                let folded = ConstantFolder::fold(&evaluated);
                let as_str = PrettyPrinter::print_with_context(&folded, var_map.as_context());
                println!("\t{}", as_str);
            }

            TopLevelExpression::Assignment(var, expr) => {
                let as_str = PrettyPrinter::print_with_context(&expr, var_map.as_context());
                let name = var_map.var(var).unwrap();
                println!("\tassigning to: {}", name);
                println!("\tthe value: {}", as_str);
                var_context.insert(var, expr);
            }
        }

        new_line();
    }
}
