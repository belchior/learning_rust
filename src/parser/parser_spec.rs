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
fn should_parse_expression_with_brackets() {
  // formula = 1*(2+3)÷5

  let tokens = vec![
    Token::new_number(vec![One]),
    Token::new_operator(Multiplication),
    Token::new_bracket(RoundOpen),
    Token::new_number(vec![Two]),
    Token::new_operator(Addition),
    Token::new_number(vec![Three]),
    Token::new_bracket(RoundClose),
    Token::new_operator(Division),
    Token::new_number(vec![Five]),
  ];

  let expected_ast = Ast::new(
    Ast::new_operator(Division),
    Ast::node_ast(Ast::new(
      Ast::new_operator(Multiplication),
      Ast::new_number(vec![One]),
      Ast::node_ast(Ast::new(
        Ast::new_operator(Addition),
        Ast::new_number(vec![Two]),
        Ast::new_number(vec![Three]),
      )),
    )),
    Ast::new_number(vec![Five]),
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

#[test]
fn testing_bracket_expression() {
  // formula = {[({1})]}+1

  let tokens = vec![
    Token::new_bracket(CurlyOpen),
    Token::new_bracket(BoxOpen),
    Token::new_bracket(RoundOpen),
    Token::new_bracket(CurlyOpen),
    Token::new_number(vec![One]),
    Token::new_bracket(CurlyClose),
    Token::new_bracket(RoundClose),
    Token::new_bracket(BoxClose),
    Token::new_bracket(CurlyClose),
    Token::new_operator(Addition),
    Token::new_number(vec![One]),
  ];
  let expected_expression = vec![
    Token::new_bracket(CurlyOpen),
    Token::new_bracket(BoxOpen),
    Token::new_bracket(RoundOpen),
    Token::new_bracket(CurlyOpen),
    Token::new_number(vec![One]),
    Token::new_bracket(CurlyClose),
    Token::new_bracket(RoundClose),
    Token::new_bracket(BoxClose),
    Token::new_bracket(CurlyClose),
  ];
  let result = bracket_expression(tokens);

  assert_eq!(
    result,
    Ok(expected_expression),
    "should match the most outer pair of bracket"
  );
}

#[test]
fn testing_error_bracket_expression() {
  let tokens = vec![Token::new_number(vec![Nine])];
  let result = bracket_expression(tokens);
  let expected_result = Err(Error::InvalidExpression(
    "Bad format: Is expected a bracket at this point".to_string(),
  ));

  assert_eq!(result, expected_result)
}

#[test]
fn testing_bracket_close_of() {
  assert_eq!(
    bracket_close_of(&Token::new_bracket(CurlyOpen)),
    Some(Token::new_bracket(CurlyClose))
  );
  assert_eq!(
    bracket_close_of(&Token::new_bracket(BoxOpen)),
    Some(Token::new_bracket(BoxClose))
  );
  assert_eq!(
    bracket_close_of(&Token::new_bracket(RoundOpen)),
    Some(Token::new_bracket(RoundClose))
  );
  assert_eq!(bracket_close_of(&Token::new_number(vec![Seven])), None);
}

#[test]
fn testing_remove_border_bracket() {
  let tokens = vec![
    Token::new_bracket(CurlyOpen),
    Token::new_number(vec![One]),
    Token::new_bracket(CurlyClose),
  ];
  let expected_expression: Vec<Token> = vec![Token::new_number(vec![One])];
  let expression = remove_border_bracket(tokens);

  assert_eq!(expression, expected_expression);
}
