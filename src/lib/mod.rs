pub mod parser;

pub fn run(file: &str) {
    let ast = parser::parse_program(file).unwrap();

    println!("{:#?}", ast.1);
}
