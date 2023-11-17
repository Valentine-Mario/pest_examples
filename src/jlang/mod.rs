use std::ffi::CString;
use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/jlang.pest"]
pub struct JLangParser;

pub enum MonadicVerb{
    Increment,
    Square,
    Negate,
    Reciprocal,
    Tally,
    Ceiling,
    ShapeOf,
}

pub enum DyadicVerb{
    Plus,
    Times,
    LessThan,
    LargerThan,
    Equal,
    Minus,
    Divide,
    Power,
    Residue,
    Copy,
    LargerOf,
    LargerOrEqual,
    Shape,
}

pub enum AstNode {
    Print(Box<AstNode>),
    Integer(i32),
    DoublePrecisionFloat(f64),
    MonadicOp{
        verb: MonadicVerb,
        expr: Box<AstNode>
    },
    DyadicOp{
        verb: DyadicVerb,
        lhs:Box<AstNode>,
        rhs:Box<AstNode>
    },
    Terms(Vec<AstNode>),
    IsGlobal{
        ident: String,
        expr: Box<AstNode>
    },
    Ident(String),
    Str(CString),
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>)->AstNode{
    AstNode::Integer(40)
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>)-> AstNode {
        match pair.as_rule() {
            Rule::integer=>{
                let istr=pair.as_str();
                let (sign, int_string)=match &istr[..1] {
                    "_"=>(-1, &istr[1..]),
                    _=>(1, &istr[..])
                };
                let integer:i32=int_string.parse().unwrap();
                AstNode::Integer(sign*integer)
            },
            Rule::decimal=>{
                let dstr=pair.as_str();
                let (sign, dec_string)= match &dstr[..1] {
                    "_"=>(-1.0, &dstr[1..]),
                    _=>(1.0, &dstr[..])
                };
                let mut float:f64=dec_string.parse().unwrap();
                if float!=0.0{
                    //avoid negative zero
                    float *= sign
                }
                AstNode::DoublePrecisionFloat(float)
            },
            Rule::expr=>{
                build_ast_from_expr(pair)
            },
            Rule::ident=>{
                AstNode::Ident(String::from(pair.as_str()))
            },
            unknown_term => panic!("Unexpected term: {:?}", unknown_term),
        }
}