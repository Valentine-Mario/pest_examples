use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/ini.pest"]
pub struct IniParser;
