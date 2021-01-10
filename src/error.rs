#[derive(Debug)]
pub enum Error {
  UndefinedSymbol(String),
  InvalidExpression,
}
