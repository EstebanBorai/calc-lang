use super::parser::{
    Expression, ExpressionOperator, Factor, Program, Statement, Term, TermOperator,
};
use super::symbol_table::SymbolTable;
use super::Result;

pub type AnalyzedTerm = (AnalyzedFactor, Vec<(TermOperator, AnalyzedFactor)>);
pub type AnalyzedExpression = (AnalyzedTerm, Vec<(ExpressionOperator, AnalyzedTerm)>);

#[derive(Debug, PartialEq)]
pub enum AnalyzedFactor {
    Identifier(usize),
    Literal(f64),
    SubExpression(Box<AnalyzedExpression>),
}

#[derive(Debug)]
pub enum AnalyzedStatement {
    Declaration(usize),
    InputOperation(usize),
    OutputOperation(AnalyzedExpression),
    Assignment(usize, AnalyzedExpression),
}

pub type AnalyzedProgram = Vec<AnalyzedStatement>;

fn analyze_factor(variables: &mut SymbolTable, factor: &Factor) -> Result<AnalyzedFactor> {
    match &factor {
        Factor::Literal(value) => Ok(AnalyzedFactor::Literal(*value)),
        Factor::Identifier(name) => {
            Ok(AnalyzedFactor::Identifier(variables.index_of_symbol(name)?))
        }
        Factor::SubExpression(expression) => Ok(AnalyzedFactor::SubExpression(Box::new(
            analyze_expression(variables, &expression)?,
        ))),
    }
}

fn analyze_term(variables: &mut SymbolTable, term: &Term) -> Result<AnalyzedTerm> {
    let first_factor = analyze_factor(variables, &term.0)?;
    let mut remaining_factors: Vec<(TermOperator, AnalyzedFactor)> = Vec::new();

    for (term_operator, factor) in &term.1 {
        remaining_factors.push((*term_operator, analyze_factor(variables, &factor)?));
    }

    Ok((first_factor, remaining_factors))
}

fn analyze_expression(
    variables: &mut SymbolTable,
    expression: &Expression,
) -> Result<AnalyzedExpression> {
    let first_term = analyze_term(variables, &expression.0)?;
    let mut remaining_terms: Vec<(ExpressionOperator, AnalyzedTerm)> = Vec::new();

    for (expression_operator, factors) in &expression.1 {
        remaining_terms.push((*expression_operator, analyze_term(variables, &factors)?));
    }

    Ok((first_term, remaining_terms))
}

fn analyze_statement(
    variables: &mut SymbolTable,
    statement: &Statement,
) -> Result<AnalyzedStatement> {
    match &statement {
        Statement::Assignment(identifier, expression) => {
            let symbol_index = variables.index_of_symbol(identifier)?;
            let analyzed_expression = analyze_expression(variables, &expression)?;

            Ok(AnalyzedStatement::Assignment(
                symbol_index,
                analyzed_expression,
            ))
        }
        Statement::Declaration(identifier) => {
            let handle = variables.insert_symbol(identifier)?;
            Ok(AnalyzedStatement::Declaration(handle))
        }
        Statement::Input(identifier) => {
            let handle = variables.index_of_symbol(identifier)?;
            Ok(AnalyzedStatement::InputOperation(handle))
        }
        Statement::Output(expr) => {
            let analyzed_expr = analyze_expression(variables, expr)?;
            Ok(AnalyzedStatement::OutputOperation(analyzed_expr))
        }
    }
}

pub fn analyze_program(
    variables: &mut SymbolTable,
    parsed_program: &Program,
) -> Result<AnalyzedProgram> {
    let mut analyzed_program = AnalyzedProgram::new();

    for statement in parsed_program {
        analyzed_program.push(analyze_statement(variables, statement)?);
    }

    Ok(analyzed_program)
}
