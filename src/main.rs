use std::{collections::HashMap, fs};

use pest::{iterators::Pair, Parser};

use crate::csv::{CSVParser, Rule};
use crate::ini::{IniParser, Rule as IniRule};
use crate::json::{parse_json_file, serialize_jsonvalue, JSONValue};
use crate::toml::{parse_toml, Rule as TOMLRule, TOMLParser};

pub mod csv;
pub mod ini;
pub mod jlang;
pub mod json;
pub mod toml;
fn main() {
    //parse json
    // let unparsed_file = fs::read_to_string("data.json").expect("cannot read file");

    // let json_parsed: JSONValue = parse_json_file(&unparsed_file).expect("unsuccessful parse");

    // println!(
    //     "serialized: {}\n parsed_json: {:?}\n",
    //     serialize_jsonvalue(&json_parsed),
    //     json_parsed
    // );

    let unparsed_toml = fs::read_to_string("data.toml").expect("cannot read file");
    let toml_rule = TOMLParser::parse(TOMLRule::toml, &unparsed_toml)
        .unwrap()
        .next()
        .unwrap();
    let output = vec![];
    let toml_ast: Result<Vec<toml::TOMLAst<'_>>, pest::error::Error<TOMLRule>> =
        parse_toml(toml_rule, output);
    println!("AST: {:?}", toml_ast);
    // let successful_parse = csv::CSVParser::parse(Rule::field, "-273.15");
    // println!("{:?}", successful_parse);

    // let unsuccessful_parse = csv::CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);

    // //parse ini file
    // let unparsed_ini_file = fs::read_to_string("file.ini").expect("error reading ini file");
    // let file = IniParser::parse(IniRule::file, &unparsed_ini_file)
    //     .expect("error parsing file")
    //     .next()
    //     .unwrap();
    // let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    // let mut current_section_name = "";

    // for line in file.into_inner() {
    //     match line.as_rule() {
    //         IniRule::section => {
    //             let mut inner_rule = line.into_inner(); // {name}
    //             current_section_name = inner_rule.next().unwrap().as_str();
    //             properties.insert(current_section_name, HashMap::new()); //initinialize key with an empty hash map
    //         }
    //         IniRule::property => {
    //             let mut inner_rule = line.into_inner(); // {name = value}
    //             let name: &str = inner_rule.next().unwrap().as_str();
    //             let value: &str = inner_rule.next().unwrap().as_str();

    //             let section = properties.entry(current_section_name).or_default();
    //             section.insert(name, value);
    //         }
    //         IniRule::EOI => {}
    //         _ => unreachable!(),
    //     }
    // }
    // println!("{:#?}", properties);

    // //parser to organize csv fields by line number
    // let record_count: u64 = 0;
    // let field_value: Vec<String> = vec![];
    // let output: HashMap<u64, Vec<String>> = HashMap::new();

    // let unparsed_csv = fs::read_to_string("numbers.csv").expect("error reading file");
    // let file = CSVParser::parse(Rule::file, &unparsed_csv)
    //     .expect("error parsing csv")
    //     .next()
    //     .unwrap();
    // let parsed_csv = parse_csv_to_map(file, record_count, field_value, output);
    // println!("PARSED CSV TO MAP: {:?}", parsed_csv);
}

fn parse_csv_to_map(
    file: Pair<'_, Rule>,
    mut record_count: u64,
    mut field_value: Vec<String>,
    mut output: HashMap<u64, Vec<String>>,
) -> HashMap<u64, Vec<String>> {
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;

                for field in record.into_inner() {
                    field_value.push(field.as_str().to_string());
                    output.insert(record_count, (*field_value).to_vec());
                }
                field_value.clear();
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    output
}
