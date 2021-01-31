use super::*;
use crate::tokenizer::Key;
use k9::assert_equal;

#[test]
fn testing_run() {
  let ast = Ast::new(
    Ast::new_operator(Key::Addition),
    Ast::new_number(vec![Key::One]),
    Ast::new_number(vec![Key::Two]),
  );

  let result = run(Ok(ast));

  assert_equal!(result, Err(Error::InvalidExpression));
}
