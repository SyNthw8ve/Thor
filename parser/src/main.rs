use parser::parse;


fn main() {
    let source = "SELECT c.g as t, d.*, f FROM snowflake.table";
    parse(source);
}