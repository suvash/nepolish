use std::result::Result;

use pest::error::Error as PError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "parser/polish/grammar.pest"]
struct PolishParser;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Node {
    Int(u32),
    Expr { op: Operator, children: Vec<Node> },
}

pub fn parse(source: &str) -> Result<Node, PError<Rule>> {
    let polish = PolishParser::parse(Rule::polish, source)?.next().unwrap();
    let ast = parse_notation(polish);
    println!("{:#?}", &ast);
    Ok(ast)
}

fn parse_notation(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::notation => {
            let mut pairs = pair.into_inner();
            let oper = pairs.next().unwrap();
            let exprs = pairs;

            let operator = parse_operator(oper);
            let expressions = parse_expressions(exprs);

            Node::Expr {
                op: operator,
                children: expressions,
            }
        }
        _ => unreachable!(),
    }
}

fn parse_operator(pair: Pair<Rule>) -> Operator {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::add => Operator::Add,
        Rule::subtract => Operator::Subtract,
        Rule::multiply => Operator::Multiply,
        Rule::divide => Operator::Divide,
        _ => unreachable!(),
    }
}

fn parse_expressions(pairs: Pairs<Rule>) -> Vec<Node> {
    let mut exprs: Vec<Node> = vec![];

    for pair in pairs {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::num => {
                let num = pair.as_str().parse::<u32>().unwrap();
                exprs.push(Node::Int(num))
            }
            Rule::notation => exprs.push(parse_notation(pair)),
            _ => unreachable!(),
        }
    }

    exprs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        parse("+ 14 (* 23 34 45) (/ 100 2 3) (- 23 1)");
        assert_eq!(1 + 1, 1)
    }
}
