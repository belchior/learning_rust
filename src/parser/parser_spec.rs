use super::*;
use pretty_assertions::assert_eq;
use Key::*;

#[test]
fn should_parse_integer_number() {
  let tokens = vec![Token::new_number(vec![One])];

  let expected_ast = Ast::new(
    Ast::new_operator(Addition),
    Ast::new_number(vec![Zero]),
    Ast::new_number(vec![One]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_integer_number_started_with_addition_sign() {
  let tokens = vec![Token::new_operator(Addition), Token::new_number(vec![Two])];

  let expected_ast = Ast::new(
    Ast::new_operator(Addition),
    Ast::new_number(vec![Zero]),
    Ast::new_number(vec![Two]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_integer_number_started_with_subtraction_sign() {
  let tokens = vec![Token::new_operator(Subtraction), Token::new_number(vec![Three])];

  let expected_ast = Ast::new(
    Ast::new_operator(Subtraction),
    Ast::new_number(vec![Zero]),
    Ast::new_number(vec![Three]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_floating_point_number() {
  let tokens = vec![Token::new_number(vec![One, Dot, Five])];

  let expected_ast = Ast::new(
    Ast::new_operator(Addition),
    Ast::new_number(vec![Zero]),
    Ast::new_number(vec![One, Dot, Five]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_floating_point_number_started_with_addition_sign() {
  let tokens = vec![Token::new_operator(Addition), Token::new_number(vec![Zero, Dot, Five])];

  let expected_ast = Ast::new(
    Ast::new_operator(Addition),
    Ast::new_number(vec![Zero]),
    Ast::new_number(vec![Zero, Dot, Five]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_floating_point_number_started_with_subtration_sign() {
  let tokens = vec![
    Token::new_operator(Subtraction),
    Token::new_number(vec![Zero, Dot, Five]),
  ];

  let expected_ast = Ast::new(
    Ast::new_operator(Subtraction),
    Ast::new_number(vec![Zero]),
    Ast::new_number(vec![Zero, Dot, Five]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_addition_expression() {
  let tokens = vec![
    Token::new_number(vec![One]),
    Token::new_operator(Addition),
    Token::new_number(vec![One]),
  ];

  let expected_ast = Ast::new(
    Ast::new_operator(Addition),
    Ast::new_number(vec![One]),
    Ast::new_number(vec![One]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_subtraction_expression() {
  let tokens = vec![
    Token::new_number(vec![Two]),
    Token::new_operator(Subtraction),
    Token::new_number(vec![Two]),
  ];

  let expected_ast = Ast::new(
    Ast::new_operator(Subtraction),
    Ast::new_number(vec![Two]),
    Ast::new_number(vec![Two]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_division_expression() {
  let tokens = vec![
    Token::new_number(vec![Three]),
    Token::new_operator(Division),
    Token::new_number(vec![Three]),
  ];

  let expected_ast = Ast::new(
    Ast::new_operator(Division),
    Ast::new_number(vec![Three]),
    Ast::new_number(vec![Three]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_multiplication_expression() {
  let tokens = vec![
    Token::new_number(vec![Four]),
    Token::new_operator(Multiplication),
    Token::new_number(vec![Four]),
  ];

  let expected_ast = Ast::new(
    Ast::new_operator(Multiplication),
    Ast::new_number(vec![Four]),
    Ast::new_number(vec![Four]),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn should_parse_multi_operation_expression() {
  // formula = 1+2-3*4/6

  let tokens = vec![
    Token::new_number(vec![One]),
    Token::new_operator(Addition),
    Token::new_number(vec![Two]),
    Token::new_operator(Subtraction),
    Token::new_number(vec![Three]),
    Token::new_operator(Multiplication),
    Token::new_number(vec![Four]),
    Token::new_operator(Division),
    Token::new_number(vec![Six]),
  ];

  let expected_ast = Ast::new(
    Ast::new_operator(Subtraction),
    Ast::node_ast(Ast::new(
      Ast::new_operator(Addition),
      Ast::new_number(vec![One]),
      Ast::new_number(vec![Two]),
    )),
    Ast::node_ast(Ast::new(
      Ast::new_operator(Division),
      Ast::node_ast(Ast::new(
        Ast::new_operator(Multiplication),
        Ast::new_number(vec![Three]),
        Ast::new_number(vec![Four]),
      )),
      Ast::new_number(vec![Six]),
    )),
  );

  let ast = parse(Ok(tokens));

  assert_eq!(ast, Ok(expected_ast));
}

#[test]
fn testing_remove_space() {
  let tokens = vec![
    Token::new_number(vec![One]),
    Token::new_space(vec![Space, Space, Space]),
    Token::new_operator(Addition),
    Token::new_space(vec![Space, Space, Space]),
    Token::new_number(vec![Two]),
  ];

  let expected_tokens = vec![
    Token::new_number(vec![One]),
    Token::new_operator(Addition),
    Token::new_number(vec![Two]),
  ];

  let tokens_without_space = remove_space(tokens);
  assert_eq!(tokens_without_space, expected_tokens);
}
