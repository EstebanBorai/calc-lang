pub mod analyzer;
pub mod interpreter;
pub mod parser;
pub mod repl;
pub mod symbol_table;

use self::analyzer::analyze_program;
use self::interpreter::exec;
use self::parser::parse_program;
use self::symbol_table::SymbolTable;

pub type Result<T> = std::result::Result<T, String>;

pub fn run_source(source: &str) -> Result<()> {
    let mut parsed_program = parse_program(source).unwrap();
    let mut variables = SymbolTable::new();
    let analysis = analyze_program(&mut variables, &mut parsed_program.1)?;

    interpreter::exec(&mut variables, &analysis);

    Ok(())
}

pub fn run_interactive(variables: &mut SymbolTable, statement: &str) -> Result<()> {
    let (remain, parsed) = parse_program(statement).map_err(|e| e.to_string())?;

    if remain.len() > 0 {
        return Err(format!(
            "Statement not recognized as a valid Calc statement: {}",
            remain
        ));
    }

    let analysis = analyze_program(variables, &parsed)?;

    exec(variables, &analysis);

    Ok(())
}
