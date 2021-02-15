use crate::parser::{parse, Ast, Node};
use crate::tokenizer::{tokenize, Key, Kind, Token};
use crate::Error;

#[rustfmt::skip]
pub fn run(formula: String) -> Result<String, Error> {
  Ok(formula)
    .map(tokenize)
    .map(parse)
    .map(interpret)
    .map(format)
}

fn interpret(ast: Result<Ast, Error>) -> Result<f64, Error> {
  let Ast {
    operator,
    operand_a,
    operand_b,
  } = ast?;

  let value_a = match operand_a {
    Some(Node::Ast(boxed_ast)) => interpret(Ok(*boxed_ast))?,
    Some(Node::Token(token)) => to_float(token)?,
    None => return Err(Error::InvalidOperand),
  };
  let value_b = match operand_b {
    Some(Node::Ast(boxed_ast)) => interpret(Ok(*boxed_ast))?,
    Some(Node::Token(token)) => to_float(token)?,
    None => return Err(Error::InvalidOperand),
  };

  basic_operation(operator.unwrap(), value_a, value_b)
}

fn format(num: Result<f64, Error>) -> String {
  let value = num.unwrap();
  if value.fract() == 0.0 {
    return (value as isize).to_string();
  }
  value.to_string()
}

fn to_float(token: Token) -> Result<f64, Error> {
  if token.kind != Kind::Number {
    return Err(Error::TokenNaN);
  }
  let keys_str = token.keys.iter().fold("".to_string(), |mut acc, key| {
    acc.push_str(&key.to_string());
    acc
  });

  Ok(keys_str.parse().unwrap())
}

fn basic_operation(operator: Token, value_a: f64, value_b: f64) -> Result<f64, Error> {
  match operator.keys[0] {
    Key::Multiplication => multiplication(value_a, value_b),
    Key::Division => division(value_a, value_b),
    Key::Addition => addition(value_a, value_b),
    Key::Subtraction => subtraction(value_a, value_b),
    _ => Err(Error::TokenNaO),
  }
}

fn multiplication(value_a: f64, value_b: f64) -> Result<f64, Error> {
  Ok(value_a * value_b)
}

fn division(value_a: f64, value_b: f64) -> Result<f64, Error> {
  if value_b == 0.0 {
    return Err(Error::DivisionByZero);
  }
  Ok(value_a / value_b)
}

fn addition(value_a: f64, value_b: f64) -> Result<f64, Error> {
  Ok(value_a + value_b)
}

fn subtraction(value_a: f64, value_b: f64) -> Result<f64, Error> {
  Ok(value_a - value_b)
}

#[cfg(test)]
mod interpreter_spec;
