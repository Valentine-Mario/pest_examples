use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/json.pest"]
struct JSONParser;

pub fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::json, file)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => unreachable!(),
        }
    }
    Ok(parse_value(json))
}

//define AST for JSON
#[derive(Debug)]
pub enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

pub fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue::*;
    match val {
        Object(obj) => {
            let content: Vec<_> = obj
                .iter()
                .map(|(key, value)| format!("\"{}\":{}", key, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", content.join(","))
        }
        Array(arr) => {
            let content: Vec<_> = arr.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", content.join(","))
        }
        String(str) => {
            format!("\"{}\"", str)
        }
        Number(num) => {
            format!("{}", num)
        }
        Boolean(bool) => {
            format!("{}", bool)
        }
        Null => format!("null"),
    }
}
