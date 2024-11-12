use rustpython_parser::{ast, Parse};
use std::fs;
fn main() {
    // let python_source = "print('Hello world')";
    let python_source =
        fs::read_to_string("charm/src/charm.py").expect("Should have been able to read the file");
    // println!("{}", python_source);
    let python_statements = ast::Suite::parse(&python_source, "").unwrap();
    // let python_expr = ast::Expr::parse(&python_source, "").unwrap();
    println!("{:?}", python_statements);
    // println!("{:?}", python_expr);
}
