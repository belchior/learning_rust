use super::*;
use crate::tokenizer::Key;
use pretty_assertions::assert_eq;

#[test]
fn testing_basic_operation_calling_multiplication() {
  let operator = Token::new_operator(Key::Multiplication);
  let operand_a = Token::new_number(vec![Key::Six]);
  let operand_b = Token::new_number(vec![Key::Seven]);
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(
    result,
    Ok(42.0),
    "should calculate tokens using the multiplication function"
  );
}

#[test]
fn testing_basic_operation_calling_division() {
  let operator = Token::new_operator(Key::Division);
  let operand_a = Token::new_number(vec![Key::Two, Key::One, Key::Zero]);
  let operand_b = Token::new_number(vec![Key::Five]);
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(result, Ok(42.0), "should calculate tokens using the division function");
}

#[test]
fn testing_basic_operation_calling_addiction() {
  let operator = Token::new_operator(Key::Addition);
  let operand_a = Token::new_number(vec![Key::One, Key::Seven]);
  let operand_b = Token::new_number(vec![Key::Two, Key::Five]);
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(result, Ok(42.0), "should calculate tokens using the addition function");
}

#[test]
fn testing_basic_operation_calling_subtraction() {
  let operator = Token::new_operator(Key::Subtraction);
  let operand_a = Token::new_number(vec![Key::Seven, Key::Eight]);
  let operand_b = Token::new_number(vec![Key::Three, Key::Six]);
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(
    result,
    Ok(42.0),
    "should calculate tokens using the subtraction function"
  );
}

#[test]
fn testing_basic_operation_returning_error() {
  let operator = Token::new_number(vec![Key::Zero]);
  let operand_a = Token::new_number(vec![Key::Seven, Key::Eight]);
  let operand_b = Token::new_number(vec![Key::Three, Key::Six]);
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(
    result,
    Err(Error::TokenNaO),
    "should return the error: Token is not an operator"
  );
}

#[test]
fn testing_to_float() {
  let token_01 = Token::new_number(vec![Key::One]);
  let token_02 = Token::new_number(vec![Key::One, Key::Dot, Key::Five]);
  let token_03 = Token::new_operator(Key::Multiplication);

  assert_eq!(to_float(token_01), Ok(1.0), "should convert token to float 1.0");
  assert_eq!(to_float(token_02), Ok(1.5), "should convert token to float 1.5");
  assert_eq!(
    to_float(token_03),
    Err(Error::TokenNaN),
    "should return an error when token is not a number"
  );
}

#[test]
fn testing_multiplication() {
  let token_a = Token::new_number(vec![Key::Two]);
  let token_b = Token::new_number(vec![Key::Eight]);

  let result = multiplication(token_a, token_b);
  assert_eq!(result, Ok(16.0));
}

#[test]
fn testing_division() {
  let token_a = Token::new_number(vec![Key::Five]);
  let token_b = Token::new_number(vec![Key::Two]);

  let result = division(token_a, token_b);
  assert_eq!(result, Ok(2.5));
}

#[test]
fn testing_division_by_zero() {
  let token_a = Token::new_number(vec![Key::Five]);
  let token_b = Token::new_number(vec![Key::Zero]);

  let result = division(token_a, token_b);
  assert_eq!(result, Err(Error::DivisionByZero));
}

#[test]
fn testing_addition() {
  let token_a = Token::new_number(vec![Key::Four, Key::Zero]);
  let token_b = Token::new_number(vec![Key::Two]);

  let result = addition(token_a, token_b);
  assert_eq!(result, Ok(42.0));
}

#[test]
fn testing_subtraction() {
  let token_a = Token::new_number(vec![Key::Four, Key::Zero]);
  let token_b = Token::new_number(vec![Key::Two]);

  let result = subtraction(token_a, token_b);
  assert_eq!(result, Ok(38.0));
}
