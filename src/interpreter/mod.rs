use crate::parser::Ast;
use crate::Error;

#[derive(Debug, PartialEq)]
pub struct Number;

pub fn run(ast: Result<Ast, Error>) -> Result<Number, Error> {
  Err(Error::InvalidExpression)
}

#[cfg(test)]
mod interpreter_spec;
