use rustpython_parser::{ast, Parse};
fn main() {
    let python_source = "print('Hello world')";
    let python_statements = ast::Suite::parse(python_source, "").unwrap();
    let python_expr = ast::Expr::parse(python_source, "").unwrap();
    println!("{:?}", python_statements);
    println!("{:?}", python_expr);
}
