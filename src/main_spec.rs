use super::*;

fn s(t: &str) -> String {
  t.to_string()
}

#[test]
fn should_calculate_solo_positive_numbers() {
  // integer
  assert_eq!(Calc::calculate(s("0")), Ok(s("0")));
  assert_eq!(Calc::calculate(s("1")), Ok(s("1")));
  assert_eq!(Calc::calculate(s("2")), Ok(s("2")));
  assert_eq!(Calc::calculate(s("3")), Ok(s("3")));
  assert_eq!(Calc::calculate(s("4")), Ok(s("4")));
  assert_eq!(Calc::calculate(s("5")), Ok(s("5")));
  assert_eq!(Calc::calculate(s("6")), Ok(s("6")));
  assert_eq!(Calc::calculate(s("7")), Ok(s("7")));
  assert_eq!(Calc::calculate(s("8")), Ok(s("8")));
  assert_eq!(Calc::calculate(s("9")), Ok(s("9")));

  // float
  assert_eq!(Calc::calculate(s("0.0")), Ok(s("0")));
  assert_eq!(Calc::calculate(s("1.0")), Ok(s("1")));
  assert_eq!(Calc::calculate(s("2.0")), Ok(s("2")));
  assert_eq!(Calc::calculate(s("3.0")), Ok(s("3")));
  assert_eq!(Calc::calculate(s("4.0")), Ok(s("4")));
  assert_eq!(Calc::calculate(s("5.0")), Ok(s("5")));
  assert_eq!(Calc::calculate(s("6.0")), Ok(s("6")));
  assert_eq!(Calc::calculate(s("7.0")), Ok(s("7")));
  assert_eq!(Calc::calculate(s("8.0")), Ok(s("8")));
  assert_eq!(Calc::calculate(s("9.0")), Ok(s("9")));
}

#[test]
fn should_calculate_solo_negative_number() {
  // integer
  assert_eq!(Calc::calculate(s("-0")), Ok(s("0")));
  assert_eq!(Calc::calculate(s("-1")), Ok(s("-1")));
  assert_eq!(Calc::calculate(s("-2")), Ok(s("-2")));
  assert_eq!(Calc::calculate(s("-3")), Ok(s("-3")));
  assert_eq!(Calc::calculate(s("-4")), Ok(s("-4")));
  assert_eq!(Calc::calculate(s("-5")), Ok(s("-5")));
  assert_eq!(Calc::calculate(s("-6")), Ok(s("-6")));
  assert_eq!(Calc::calculate(s("-7")), Ok(s("-7")));
  assert_eq!(Calc::calculate(s("-8")), Ok(s("-8")));
  assert_eq!(Calc::calculate(s("-9")), Ok(s("-9")));

  // float
  assert_eq!(Calc::calculate(s("-0.0")), Ok(s("0")));
  assert_eq!(Calc::calculate(s("-1.0")), Ok(s("-1")));
  assert_eq!(Calc::calculate(s("-2.0")), Ok(s("-2")));
  assert_eq!(Calc::calculate(s("-3.0")), Ok(s("-3")));
  assert_eq!(Calc::calculate(s("-4.0")), Ok(s("-4")));
  assert_eq!(Calc::calculate(s("-5.0")), Ok(s("-5")));
  assert_eq!(Calc::calculate(s("-6.0")), Ok(s("-6")));
  assert_eq!(Calc::calculate(s("-7.0")), Ok(s("-7")));
  assert_eq!(Calc::calculate(s("-8.0")), Ok(s("-8")));
  assert_eq!(Calc::calculate(s("-9.0")), Ok(s("-9")));
}

#[test]
fn should_calculate_four_basic_operation() {
  assert_eq!(Calc::calculate(s("1+2")), Ok(s("3")));
  assert_eq!(Calc::calculate(s("3-4")), Ok(s("-1")));
  assert_eq!(Calc::calculate(s("5*6")), Ok(s("30")));
  assert_eq!(Calc::calculate(s("7/8")), Ok(s("0.875")));
}

#[test]
fn should_calculate_three_types_of_brackets() {
  assert_eq!(Calc::calculate(s("(0)")), Ok(s("0")));
  assert_eq!(Calc::calculate(s("[2]")), Ok(s("2")));
  assert_eq!(Calc::calculate(s("{4}")), Ok(s("4")));
}

#[test]
fn should_calculate_respecting_precedence_order_of_basic_operators() {
  // precedence order: * = / > + = -

  assert_eq!(Calc::calculate(s("1+2+3")), Ok(s("6")));
  assert_eq!(Calc::calculate(s("1+2-3")), Ok(s("0")));
  assert_eq!(Calc::calculate(s("4+2/5")), Ok(s("4.4")));
  assert_eq!(Calc::calculate(s("1+2*3")), Ok(s("7")));

  assert_eq!(Calc::calculate(s("1-2+3")), Ok(s("2")));
  assert_eq!(Calc::calculate(s("1-2-3")), Ok(s("-4")));
  assert_eq!(Calc::calculate(s("4-2/5")), Ok(s("3.6")));
  assert_eq!(Calc::calculate(s("1-2*3")), Ok(s("-5")));

  assert_eq!(Calc::calculate(s("1/2+3")), Ok(s("3.5")));
  assert_eq!(Calc::calculate(s("1/2-3")), Ok(s("-2.5")));
  assert_eq!(Calc::calculate(s("1/2/5")), Ok(s("0.1")));
  assert_eq!(Calc::calculate(s("2/2*3")), Ok(s("3")));

  assert_eq!(Calc::calculate(s("3*2+3")), Ok(s("9")));
  assert_eq!(Calc::calculate(s("3*2-3")), Ok(s("3")));
  assert_eq!(Calc::calculate(s("3*2/5")), Ok(s("1.2")));
  assert_eq!(Calc::calculate(s("3*2*3")), Ok(s("18")));
}

#[test]
fn should_calculate_brackets_before_operators() {
  assert_eq!(Calc::calculate(s("(1)")), Ok(s("1")));
  assert_eq!(Calc::calculate(s("(+1)")), Ok(s("1")));
  assert_eq!(Calc::calculate(s("(-1)")), Ok(s("-1")));

  assert_eq!(Calc::calculate(s("(2+2)")), Ok(s("4")));
  assert_eq!(Calc::calculate(s("(2)+(2)")), Ok(s("4")));
  assert_eq!(Calc::calculate(s("(2)+2")), Ok(s("4")));
  assert_eq!(Calc::calculate(s("2+(2)")), Ok(s("4")));

  assert_eq!(Calc::calculate(s("(3+3)+(3+3)")), Ok(s("12")));
  assert_eq!(Calc::calculate(s("(3+3)*(3+3)")), Ok(s("36")));
  assert_eq!(Calc::calculate(s("(3+3)*(3/3)")), Ok(s("6")));
}

#[test]
fn should_calculate_inner_brackets_before_outers() {
  assert_eq!(Calc::calculate(s("{[(1)]}")), Ok(s("1")));
  assert_eq!(Calc::calculate(s("[({1})]")), Ok(s("1")));
  assert_eq!(Calc::calculate(s("({[1]})")), Ok(s("1")));

  assert_eq!(Calc::calculate(s("((1+3)*4)")), Ok(s("16")));
  assert_eq!(Calc::calculate(s("([1+3]*4)")), Ok(s("16")));
  assert_eq!(Calc::calculate(s("({1+3}*4)")), Ok(s("16")));

  assert_eq!(Calc::calculate(s("[(1+3)*4]")), Ok(s("16")));
  assert_eq!(Calc::calculate(s("[[1+3]*4]")), Ok(s("16")));
  assert_eq!(Calc::calculate(s("[{1+3}*4]")), Ok(s("16")));

  assert_eq!(Calc::calculate(s("{(1+3)*4}")), Ok(s("16")));
  assert_eq!(Calc::calculate(s("{[1+3]*4}")), Ok(s("16")));
  assert_eq!(Calc::calculate(s("{{1+3}*4}")), Ok(s("16")));
}
