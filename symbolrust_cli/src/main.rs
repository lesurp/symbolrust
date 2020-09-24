use lazy_static::lazy_static;
use pest::{
    error::Error,
    iterators::Pair,
    iterators::Pairs,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};
use pest_derive::Parser;
use std::collections::{hash_map::Entry, HashMap};
use symbolrust_core::prelude::*;

#[derive(Parser)]
#[grammar = "../grammar.pest"] // relative to src
struct MyParser;

struct AstBuilder {
    parse_context: HashMap<String, Variable>,
    fn_to_ctor: HashMap<String, fn(Vec<Node>) -> Result<Node, ()>>,
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

impl AstBuilder {
    fn new() -> Self {
        let mut fn_to_ctor = HashMap::new();
        fn_to_ctor.insert("exp".to_owned(), Exponential::from_args as _);
        AstBuilder {
            parse_context: HashMap::new(),
            fn_to_ctor,
        }
    }

    fn build_expr<'i>(&mut self, pair: Pairs<'i, Rule>) -> Node {
        PREC_CLIMBER.climb(pair, |p| self.build_subexpr(p), infix)
    }

    fn build_subexpr<'i>(&mut self, pair: Pair<'i, Rule>) -> Node {
        match pair.as_rule() {
            Rule::expr => self.build_expr(pair.into_inner()),
            Rule::constant => pair.as_str().parse::<f64>().unwrap().into(),
            Rule::var => {
                let var_name = pair.as_str().to_owned();
                match self.parse_context.entry(var_name) {
                    Entry::Vacant(vac) => *vac.insert(Variable::new()),
                    Entry::Occupied(occ) => *occ.get(),
                }
                .into()
            }
            Rule::function_call => {
                let mut it = pair.into_inner();
                let fn_name = it.next().unwrap().as_str();
                let args = it.map(|pair| self.build_expr(pair.into_inner())).collect();
                if let Some(func) = self.fn_to_ctor.get(fn_name) {
                    func(args).unwrap()
                } else {
                    panic!("Unknown function");
                }
            }

            Rule::operation | Rule::int | Rule::num | Rule::WHITESPACE | Rule::calculation => {
                unreachable!()
            }
            Rule::add | Rule::subtract | Rule::multiply | Rule::divide | Rule::power => {
                unreachable!();
            }
            _ => {
                todo!();
            }
        }
    }
}

fn main() {
    let mut ast_builder = AstBuilder::new();

    let pairs = MyParser::parse(Rule::calculation, " x- 3*y + exp(5*z / -2.0e2)")
        .unwrap_or_else(|e| panic!("{}", e));
    let result = ast_builder.build_expr(pairs);
    let pp = PrettyPrinter::new();
    println!("Printing parsed ast: {}", result.accept_visitor(&pp));
}
