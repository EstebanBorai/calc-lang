pub mod analyzer;
pub mod parser;
pub mod symbol_table;

use self::analyzer::analyze_program;
use self::parser::parse_program;
use self::symbol_table::SymbolTable;

pub type Result<T> = std::result::Result<T, String>;

pub fn run(file: &str) -> Result<()> {
    let mut parsed_program = parse_program(file).unwrap();
    let mut variables = SymbolTable::new();
    let analysis = analyze_program(&mut variables, &mut parsed_program.1)?;

    println!("{:#?}", analysis);

    Ok(())
}
