use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/toml.pest"]
pub struct TOMLParser;

#[derive(Debug, Clone)]
pub enum TOMLAst<'a> {
    Section((&'a str, Vec<TOMLAst<'a>>)),
    ArraySection((&'a str, Vec<TOMLAst<'a>>)),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Array(Vec<TOMLAst<'a>>),
    OffsetDateTime(&'a str),
    Date(&'a str),
    Time(&'a str),
    Header(&'a str),
    ArrayHeader(&'a str),
    Pair((&'a str, Box<TOMLAst<'a>>)),
    Null(())
}

pub fn parse_toml<'a>(toml_val: Pair<'a, Rule>, mut output: Vec<TOMLAst<'a>>)->Result<Vec<TOMLAst<'a>>, Error<Rule>>{
    use TOMLAst::*;
    let toml_val=toml_val.to_owned();
    for val in toml_val.into_inner().into_iter(){
        match val.as_rule(){
            Rule::arrray_header=>{
                output.push(ArrayHeader(val.as_str()));
            },
            Rule::header=>{
                output.push(Header(val.as_str()));
            },
            
            Rule::pair=>{
                let mut pair_value=val.into_inner();
                let name=pair_value.next().unwrap().as_str();
                let value=parse_type(pair_value.next().unwrap());
               output.push(Pair((name, Box::new(value))))
            }
            _=>{
                
            }
        }
    }
    
    Ok(output.to_vec())
}

fn parse_type<'a>(toml_val: Pair<'a, Rule>)->TOMLAst<'a>{
    use TOMLAst::*;
    match toml_val.as_rule(){
        Rule::boolean=>{
            return Boolean(toml_val.as_str().parse().unwrap())
         },
         Rule::number=>{
             return Number(toml_val.as_str().parse().unwrap())
         }
         Rule::string=>{
             return String(toml_val.into_inner().next().unwrap().as_str())
         }
         Rule::offset_date_time=>{
             println!("here");
             return OffsetDateTime(toml_val.as_str())
         }
         Rule::date=>{
             return Date(toml_val.as_str())
         }
         Rule::time=>{
             return Time(toml_val.as_str())
         }
         Rule::array=>{
            return Array(toml_val.into_inner().map(parse_type).collect())
         }
         _=>{return Null(())}
    }
    
}