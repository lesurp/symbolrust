use pest::{
    error::Error,
    iterators::Pair,
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

impl AstBuilder {
    fn new() -> Self {
        let mut fn_to_ctor = HashMap::new();
        fn_to_ctor.insert("exp".to_owned(), Exponential::from_args as _);
        AstBuilder {
            parse_context: HashMap::new(),
            fn_to_ctor,
        }
    }

    fn consume<'i>(&mut self, pair: Pair<'i, Rule>) -> Node {
        match pair.as_rule() {
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
                let args = it.map(|pair| self.consume(pair)).collect();
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
                println!("parse:  {}", pair.as_str());
                unreachable!();
            }
            _ => {
                println!("parse:  {}", pair.as_str());
                todo!();
            }
        }
    }
}

fn rec_parse<'i>(pair: Pair<'i, Rule>, rec_level: usize) {
    //fn rec_parse<'i>(pair: Pair<'i, Rule>, rec_level: usize) -> Result<Node, Error<Rule>> {
    let indent = (0..rec_level).map(|_| "\t").collect::<String>();
    // A pair is a combination of the rule which matched and a span of input
    println!("{}Rule:    {:?}", indent, pair.as_rule());
    println!("{}Span:    {:?}", indent, pair.as_span());
    println!("{}Text:    {}", indent, pair.as_str());

    // A pair can be converted to an iterator of the tokens which make it up:
    let mut nodes: Vec<Node> = Vec::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => println!("{}identifier:  {}", indent, inner_pair.as_str()),
            Rule::constant => {
                nodes.push(inner_pair.as_str().parse::<f64>().unwrap().into());
                println!("{}constant:  {}", indent, inner_pair.as_str());
            }
            Rule::var => println!("{}var:  {}", indent, inner_pair.as_str()),
            Rule::add => println!("{}add:  {}", indent, inner_pair.as_str()),
            Rule::subtract => println!("{}subtract:  {}", indent, inner_pair.as_str()),
            Rule::multiply => println!("{}multiply:  {}", indent, inner_pair.as_str()),
            Rule::divide => println!("{}divide:  {}", indent, inner_pair.as_str()),
            Rule::power => println!("{}power:  {}", indent, inner_pair.as_str()),
            Rule::expr => println!("{}expr:  {}", indent, inner_pair.as_str()),
            Rule::term => rec_parse(inner_pair, rec_level + 1),
            Rule::function_call => rec_parse(inner_pair, rec_level + 1),

            Rule::operation | Rule::int | Rule::num | Rule::WHITESPACE | Rule::calculation => {
                unreachable!()
            }
        };
    }
}

fn main() {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);

    let infix = |lhs: Node, op: Pair<Rule>, rhs: Node| match op.as_rule() {
        Rule::add => lhs + rhs,
        Rule::subtract => lhs - rhs,
        Rule::multiply => lhs * rhs,
        Rule::divide => lhs / rhs,
        Rule::power => lhs ^ rhs,
        _ => unreachable!(),
    };

    let mut ast_builder = AstBuilder::new();

    let pairs = MyParser::parse(Rule::calculation, " x- 3*y + exp(5*z / -2.0e2)")
        .unwrap_or_else(|e| panic!("{}", e));
    let result = climber.climb(pairs, |a| ast_builder.consume(a), infix);

    //for pair in result {
    //rec_parse(pair, 0);
    //}
}
