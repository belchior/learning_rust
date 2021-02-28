#[derive(Debug, PartialEq)]
pub enum Error {
  DivisionByZero,
  InvalidExpression(String),
  InvalidOperand,
  InvalidTokenSequence,
  TokenNaN,
  TokenNaO,
  UndefinedSymbol(String),
}
