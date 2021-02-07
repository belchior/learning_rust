use crate::tokenizer::{Key, Kind, Token};
use crate::Error;

fn basic_operation(operator: Token, operand_a: Token, operand_b: Token) -> Result<f64, Error> {
  match operator.keys[0] {
    Key::Multiplication => multiplication(operand_a, operand_b),
    Key::Division => division(operand_a, operand_b),
    Key::Addition => addition(operand_a, operand_b),
    Key::Subtraction => subtraction(operand_a, operand_b),
    _ => Err(Error::TokenNaO),
  }
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

fn multiplication(operand_a: Token, operand_b: Token) -> Result<f64, Error> {
  let value_a = to_float(operand_a)?;
  let value_b = to_float(operand_b)?;
  Ok(value_a * value_b)
}

fn division(operand_a: Token, operand_b: Token) -> Result<f64, Error> {
  let value_a = to_float(operand_a)?;
  let value_b = to_float(operand_b)?;

  if value_b == 0.0 {
    return Err(Error::DivisionByZero);
  }

  Ok(value_a / value_b)
}

fn addition(operand_a: Token, operand_b: Token) -> Result<f64, Error> {
  let value_a = to_float(operand_a)?;
  let value_b = to_float(operand_b)?;
  Ok(value_a + value_b)
}

fn subtraction(operand_a: Token, operand_b: Token) -> Result<f64, Error> {
  let value_a = to_float(operand_a)?;
  let value_b = to_float(operand_b)?;
  Ok(value_a - value_b)
}

#[cfg(test)]
mod interpreter_spec;
