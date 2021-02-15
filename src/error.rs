#[derive(Debug, PartialEq)]
pub enum Error {
  DivisionByZero,
  InvalidExpression,
  InvalidOperand,
  InvalidTokenSequence,
  TokenNaN,
  TokenNaO,
  UndefinedSymbol(String),
}
