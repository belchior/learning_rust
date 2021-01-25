#[derive(Debug, PartialEq)]
pub enum Error {
  UndefinedSymbol(String),
  InvalidExpression,
  InvalidTokenSequence,
}
