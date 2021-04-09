use std::result::Result;

use pest::error::Error as PError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

use std::num::ParseIntError;
use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

#[derive(pest_derive::Parser)]
#[grammar = "parser/nepolish/grammar.pest"]
struct NepolishParser;

#[derive(Debug)]
struct Sanhkya {
    value: u32,
}

impl FromStr for Sanhkya {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .graphemes(true)
            .into_iter()
            .map(|n| match n {
                "०" => "0",
                "१" => "1",
                "२" => "2",
                "३" => "3",
                "४" => "4",
                "५" => "5",
                "६" => "6",
                "७" => "7",
                "८" => "8",
                "९" => "9",
                _ => n,
            })
            .collect::<Vec<&str>>()
            .concat()
            .parse::<u32>()?;

        Ok(Sanhkya { value })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Int(u32),
    Expr { op: Operator, children: Vec<Node> },
}

pub fn parse(source: &str) -> Result<Node, PError<Rule>> {
    println!("{:#?}", &source);
    let nepolish = NepolishParser::parse(Rule::nepolish, source)?
        .next()
        .unwrap();
    let ast = parse_notation(nepolish);
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
                let num = pair.as_str().parse::<Sanhkya>().unwrap();
                exprs.push(Node::Int(num.value))
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
        let input = "+ १४ (* २३ ३४ ४५) (/ १०० २ ३) (- २३ १)";
        let expected = Node::Expr {
            op: Operator::Add,
            children: vec![
                Node::Int(14),
                Node::Expr {
                    op: Operator::Multiply,
                    children: vec![Node::Int(23), Node::Int(34), Node::Int(45)],
                },
                Node::Expr {
                    op: Operator::Divide,
                    children: vec![Node::Int(100), Node::Int(2), Node::Int(3)],
                },
                Node::Expr {
                    op: Operator::Subtract,
                    children: vec![Node::Int(23), Node::Int(1)],
                },
            ],
        };
        assert_eq!(parse(input).unwrap(), expected);
    }
}
