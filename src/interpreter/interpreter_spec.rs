use super::*;
use crate::tokenizer::Key;
use pretty_assertions::assert_eq;

#[test]
fn testing_run() {
  let formula = "1+2-3*4/6".to_string();
  let result = run(formula);

  assert_eq!(result, Ok("1".to_string()), "should calculate formula 1+2 and return 3");
}

#[test]
fn testing_interpret() {
  let ast = Ast::new(
    Ast::new_operator(Key::Addition),
    Ast::new_number(vec![Key::One]),
    Ast::new_number(vec![Key::Two]),
  );
  let result = interpret(Ok(ast));

  assert_eq!(result, Ok(3.0), "should calculate formula 1+2 and return 3");
}

#[test]
fn testing_format() {
  assert_eq!(
    format(Ok(0.5)),
    "0.5".to_string(),
    "should keep as float when the number has fraction"
  );
  assert_eq!(
    format(Ok(0.0)),
    "0".to_string(),
    "should convert to integer numbers without fraction"
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
fn testing_basic_operation_calling_multiplication() {
  let operator = Token::new_operator(Key::Multiplication);
  let operand_a = 6.0;
  let operand_b = 7.0;
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
  let operand_a = 210.0;
  let operand_b = 5.0;
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(result, Ok(42.0), "should calculate tokens using the division function");
}

#[test]
fn testing_basic_operation_calling_addiction() {
  let operator = Token::new_operator(Key::Addition);
  let operand_a = 17.0;
  let operand_b = 25.0;
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(result, Ok(42.0), "should calculate tokens using the addition function");
}

#[test]
fn testing_basic_operation_calling_subtraction() {
  let operator = Token::new_operator(Key::Subtraction);
  let operand_a = 78.0;
  let operand_b = 36.0;
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
  let operand_a = 78.0;
  let operand_b = 36.0;
  let result = basic_operation(operator, operand_a, operand_b);

  assert_eq!(
    result,
    Err(Error::TokenNaO),
    "should return the error: Token is not an operator"
  );
}

#[test]
fn testing_multiplication() {
  let token_a = 2.0;
  let token_b = 8.0;

  let result = multiplication(token_a, token_b);
  assert_eq!(result, Ok(16.0));
}

#[test]
fn testing_division() {
  let token_a = 5.0;
  let token_b = 2.0;

  let result = division(token_a, token_b);
  assert_eq!(result, Ok(2.5));
}

#[test]
fn testing_division_by_zero() {
  let token_a = 5.0;
  let token_b = 0.0;

  let result = division(token_a, token_b);
  assert_eq!(result, Err(Error::DivisionByZero));
}

#[test]
fn testing_addition() {
  let token_a = 40.0;
  let token_b = 2.0;

  let result = addition(token_a, token_b);
  assert_eq!(result, Ok(42.0));
}

#[test]
fn testing_subtraction() {
  let token_a = 40.0;
  let token_b = 2.0;

  let result = subtraction(token_a, token_b);
  assert_eq!(result, Ok(38.0));
}
