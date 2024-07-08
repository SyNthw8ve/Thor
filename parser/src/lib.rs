use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "sql.pest"]
struct SQlParser;


pub fn parse(source: &str) {

    let pairs = SQlParser::parse(Rule::query, source);
    for pair in pairs {
        println!("Rule:    {:?}", pair);        
    }
}