#[derive(Debug, PartialEq)]
pub enum Error {
  DivisionByZero,
  InvalidExpression,
  InvalidTokenSequence,
  TokenNaN,
  TokenNaO,
  UndefinedSymbol(String),
}
