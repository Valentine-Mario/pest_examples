use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/toml.pest"]
pub struct TOMLParser;

#[derive(Debug, Clone)]
pub enum TOMLAst<'a> {
    Section((Vec<&'a str>, Vec<TOMLAst<'a>>)),
    ArraySection((Vec<&'a str>, Vec<TOMLAst<'a>>)),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Array(Vec<TOMLAst<'a>>),
    OffsetDateTime(&'a str),
    Date(&'a str),
    Time(&'a str),
    Header(Vec<&'a str>),
    ArrayHeader(Vec<&'a str>),
    Pair((Vec<&'a str>, Box<TOMLAst<'a>>)),
    Null(()),
}

pub fn parse_toml<'a>(
    toml_val: Pair<'a, Rule>,
    mut output: Vec<TOMLAst<'a>>,
) -> Result<Vec<TOMLAst<'a>>, Error<Rule>> {
    use TOMLAst::*;
    let toml_val = toml_val.to_owned();
    for val in toml_val.into_inner().into_iter() {
        match val.as_rule() {
            Rule::arrray_header => {
                let arr_head: Vec<&str> = val.into_inner().as_str().split(".").collect();
                output.push(ArrayHeader(arr_head));
            }
            Rule::header => {
                let head: Vec<&str> = val.into_inner().as_str().split(".").collect();
                output.push(Header(head));
            }
            Rule::section => {
                let mut pair_value = val.into_inner();
                let header = pair_value
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(".")
                    .collect();
                let mut inner_pair: Vec<TOMLAst<'a>> = vec![];
                for val in pair_value.next().unwrap().into_inner().into_iter() {
                    inner_pair.push(parse_pair(val))
                }
                output.push(Section((header, inner_pair)));
            }
            Rule::array_section => {
                let mut pair_value = val.into_inner();
                let header = pair_value
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(".")
                    .collect();
                let mut inner_pair: Vec<TOMLAst<'a>> = vec![];
                for val in pair_value.next().unwrap().into_inner().into_iter() {
                    inner_pair.push(parse_pair(val))
                }
                output.push(ArraySection((header, inner_pair)));
            }
            Rule::pair => output.push(parse_pair(val)),
            _ => {}
        }
    }

    Ok(output.to_vec())
}

fn parse_pair<'a>(toml_val: Pair<'a, Rule>) -> TOMLAst<'a> {
    let mut pair_value = toml_val.into_inner();
    let name = pair_value.next().unwrap().as_str().split(".").collect();
    let value = parse_type(pair_value.next().unwrap());
    TOMLAst::Pair((name, Box::new(value)))
}

fn parse_type<'a>(toml_val: Pair<'a, Rule>) -> TOMLAst<'a> {
    use TOMLAst::*;
    match toml_val.as_rule() {
        Rule::boolean => return Boolean(toml_val.as_str().parse().unwrap()),
        Rule::number => return Number(toml_val.as_str().parse().unwrap()),
        Rule::string => return String(toml_val.into_inner().next().unwrap().as_str()),
        Rule::offset_date_time => {
            return OffsetDateTime(toml_val.as_str());
        }
        Rule::date => return Date(toml_val.as_str()),
        Rule::time => return Time(toml_val.as_str()),
        Rule::array => return Array(toml_val.into_inner().map(parse_type).collect()),
        _ => return Null(()),
    }
}
