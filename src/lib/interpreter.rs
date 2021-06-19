use super::analyzer::{
    AnalyzedExpression, AnalyzedFactor, AnalyzedProgram, AnalyzedStatement, AnalyzedTerm,
};
use super::parser::{ExpressionOperator, TermOperator};
use super::symbol_table::SymbolTable;

fn eval_term(variables: &SymbolTable, term: &AnalyzedTerm) -> f64 {
    let mut result = eval_factor(variables, &term.0);

    for factor in &term.1 {
        match factor.0 {
            TermOperator::Multiply => result *= eval_factor(variables, &factor.1),
            TermOperator::Divide => result /= eval_factor(variables, &factor.1),
        }
    }

    result
}

fn eval_expression(variables: &SymbolTable, expression: &AnalyzedExpression) -> f64 {
    let mut result = eval_term(variables, &expression.0);

    for term in &expression.1 {
        match term.0 {
            ExpressionOperator::Add => result += eval_term(variables, &term.1),
            ExpressionOperator::Subtract => result -= eval_term(variables, &term.1),
        }
    }

    result
}

fn eval_factor(variables: &SymbolTable, factor: &AnalyzedFactor) -> f64 {
    match factor {
        AnalyzedFactor::Literal(value) => *value,
        AnalyzedFactor::Identifier(index) => variables.get(*index),
        AnalyzedFactor::SubExpression(expr) => eval_expression(variables, &expr),
    }
}

fn execute_statement(variables: &mut SymbolTable, statement: &AnalyzedStatement) {
    match statement {
        AnalyzedStatement::Assignment(handle, expr) => {
            variables.set(*handle, eval_expression(variables, expr));
        }
        AnalyzedStatement::Declaration(_) => {}
        AnalyzedStatement::InputOperation(handle) => {
            let mut text = String::new();
            eprint!("? ");
            std::io::stdin()
                .read_line(&mut text)
                .expect("Cannot read line.");
            let value = text.trim().parse::<f64>().unwrap_or(0.);
            variables.set(*handle, value);
        }
        AnalyzedStatement::OutputOperation(expr) => {
            println!("{}", eval_expression(variables, expr));
        }
    }
}

pub fn exec(variables: &mut SymbolTable, program: &AnalyzedProgram) {
    for statement in program {
        execute_statement(variables, statement);
    }
}
