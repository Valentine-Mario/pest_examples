use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/csv.pest"]
pub struct CSVParser;
