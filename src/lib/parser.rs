//! Context Free Grammar parser for the Calc Language
//!
//! The parser module is in charge of performing parsing and
//! lexical analysis on a Calc Language module.
//!
//! The output of this module is a syntax tree used by the
//! analyzer of the language.
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, char};
use nom::combinator::map;
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

const ASSIGNMENT_OPERATOR_TAG: &str = ":=";
const DECLARATION_STATEMENT_TOKEN: char = '@';
const INPUT_STATEMENT_TOKEN: char = '>';
const OUTPUT_STATEMENT_TOKEN: char = '<';
const ADDITION_OPERATOR_TOKEN: char = '+';
const SUBSTRACTION_OPERATOR_TOKEN: char = '-';
const MULTIPLICATION_OPERATOR_TOKEN: char = '*';
const DIVISION_OPERATOR_TOKEN: char = '/';
const OPEN_PARENTHESIS_TOKEN: char = '(';
const CLOSE_PARENTHESIS_TOKEN: char = ')';

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TermOperator {
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExpressionOperator {
    Add,
    Subtract,
}

#[derive(Debug)]
pub enum Statement<'a> {
    Declaration(&'a str),
    Input(&'a str),
    Output(Expression<'a>),
    Assignment(&'a str, Expression<'a>),
}

#[derive(Debug, PartialEq)]
pub enum Factor<'a> {
    Literal(f64),
    Identifier(&'a str),
    SubExpression(Box<Expression<'a>>),
}

pub type Term<'a> = (Factor<'a>, Vec<(TermOperator, Factor<'a>)>);
pub type Expression<'a> = (Term<'a>, Vec<(ExpressionOperator, Term<'a>)>);
pub type Program<'a> = Vec<Statement<'a>>;

fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";

    take_while(move |ch| chars.contains(ch))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Factor> {
    preceded(
        skip_spaces,
        alt((
            map(parse_identifier, Factor::Identifier),
            map(double, Factor::Literal),
            map(parse_subexpression, |expr| {
                Factor::SubExpression(Box::new(expr))
            }),
        )),
    )(input)
}

/// Parses subexpressions which are wrapped into a parenthesis
fn parse_subexpression(input: &str) -> IResult<&str, Expression> {
    delimited(
        preceded(skip_spaces, char(OPEN_PARENTHESIS_TOKEN)),
        parse_expression,
        preceded(skip_spaces, char(CLOSE_PARENTHESIS_TOKEN)),
    )(input)
}

/// Parses a calc term.
fn parse_term(input: &str) -> IResult<&str, Term> {
    tuple((
        parse_factor,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char(MULTIPLICATION_OPERATOR_TOKEN), |_| {
                        TermOperator::Multiply
                    }),
                    map(char(DIVISION_OPERATOR_TOKEN), |_| TermOperator::Divide),
                )),
            ),
            parse_factor,
        ))),
    ))(input)
}

/// Parses a calc expression with a valid composition.
///
/// An expression is composed by an identifier followed by an expression
/// operator such as `+` (Addition) or `-` (Substraction) tokens and finally
/// a second identifier.
///
/// Expressions are noted on Backus-Naur notation as follows:
///
/// ```ignore
/// <expr> ::= <term> | <expr> "+" <term> | <expr> "-" <term>
/// ```
///
/// Terms are parsed by the `parse_term` function.
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    tuple((
        parse_term,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char(ADDITION_OPERATOR_TOKEN), |_| ExpressionOperator::Add),
                    map(char(SUBSTRACTION_OPERATOR_TOKEN), |_| {
                        ExpressionOperator::Subtract
                    }),
                )),
            ),
            parse_term,
        ))),
    ))(input)
}

/// Parses a calc identifier which should be composed of alphabetic characters
/// only (a-z and/or A-Z)
fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

/// Parses a calc declaration statement such as `@price := 14.99`.
/// Every calc declaration statement should be preceeded by an `@` (at)
/// character, followed by alphabetic characters
///
/// The following is a valid identifier assignment:
///
/// ```ignore
/// @price := 14.99
/// ```
fn parse_declaration(input: &str) -> IResult<&str, Statement> {
    tuple((
        char(DECLARATION_STATEMENT_TOKEN),
        skip_spaces,
        parse_identifier,
    ))(input)
    .map(|(input, output)| (input, Statement::Declaration(output.2)))
}

/// Parses a calc input statement such as `> quantity`.
/// Every calc input statement should be followed by previously defined
/// identifier.
///
/// The following is a valid input statement which reads data and assigns
/// such data to the `quantity` variable
///
/// ```ignore
/// @price := 14.99
/// @quantity
///
/// > quantity
/// ```
fn parse_input_statement(input: &str) -> IResult<&str, Statement> {
    tuple((char(INPUT_STATEMENT_TOKEN), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, Statement::Input(output.2)))
}

fn parse_output_statement(input: &str) -> IResult<&str, Statement> {
    tuple((char(OUTPUT_STATEMENT_TOKEN), skip_spaces, parse_expression))(input)
        .map(|(input, output)| (input, Statement::Output(output.2)))
}

fn parse_assignment(input: &str) -> IResult<&str, Statement> {
    tuple((
        parse_identifier,
        skip_spaces,
        tag(ASSIGNMENT_OPERATOR_TAG),
        skip_spaces,
        parse_expression,
    ))(input)
    .map(|(input, output)| (input, Statement::Assignment(output.0, output.4)))
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    many0(preceded(
        skip_spaces,
        alt((
            parse_declaration,
            parse_input_statement,
            parse_output_statement,
            parse_assignment,
        )),
    ))(input)
}
