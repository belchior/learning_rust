use super::*;

#[test]
fn testing_tokenize() {
  let formula = String::from("-*/+()[]{}  1.2");

  let tokens = tokenize(formula).unwrap();

  let expected_tokens = vec![
    Token {
      kind: Kind::Operator,
      keys: vec![Key::Subtraction],
    },
    Token {
      kind: Kind::Operator,
      keys: vec![Key::Multiplication],
    },
    Token {
      kind: Kind::Operator,
      keys: vec![Key::Division],
    },
    Token {
      kind: Kind::Operator,
      keys: vec![Key::Addition],
    },
    Token {
      kind: Kind::Bracket,
      keys: vec![Key::RoundOpen],
    },
    Token {
      kind: Kind::Bracket,
      keys: vec![Key::RoundClose],
    },
    Token {
      kind: Kind::Bracket,
      keys: vec![Key::BoxOpen],
    },
    Token {
      kind: Kind::Bracket,
      keys: vec![Key::BoxClose],
    },
    Token {
      kind: Kind::Bracket,
      keys: vec![Key::CurlyOpen],
    },
    Token {
      kind: Kind::Bracket,
      keys: vec![Key::CurlyClose],
    },
    Token {
      kind: Kind::Space,
      keys: vec![Key::Space, Key::Space],
    },
    Token {
      kind: Kind::Number,
      keys: vec![Key::One, Key::Dot, Key::Two],
    },
  ];

  assert_eq!(tokens, expected_tokens);
}

#[test]
fn testing_join_spaces() {
  let formula = String::from("   ");
  let tokens = run_tokenize(formula.clone(), Vec::new(), 0).unwrap();
  let tokens_with_joined_space = join_spaces(run_tokenize(formula, Vec::new(), 0).unwrap());

  let expected_tokens = vec![
    Token {
      kind: Kind::Space,
      keys: vec![Key::Space],
    },
    Token {
      kind: Kind::Space,
      keys: vec![Key::Space],
    },
    Token {
      kind: Kind::Space,
      keys: vec![Key::Space],
    },
  ];

  let expected_tokens_with_joined_space = vec![Token {
    kind: Kind::Space,
    keys: vec![Key::Space, Key::Space, Key::Space],
  }];

  assert_eq!(tokens, expected_tokens);
  assert_eq!(tokens_with_joined_space, expected_tokens_with_joined_space);
}

#[test]
fn testing_digits_into_number() {
  let formula = String::from("123.45");
  let tokens = run_tokenize(formula.clone(), Vec::new(), 0).unwrap();
  let tokens_number = digits_into_number(run_tokenize(formula, Vec::new(), 0).unwrap());

  let expected_tokens = vec![
    Token {
      kind: Kind::Digit,
      keys: vec![Key::One],
    },
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Two],
    },
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Three],
    },
    Token {
      kind: Kind::Dot,
      keys: vec![Key::Dot],
    },
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Four],
    },
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Five],
    },
  ];

  let expected_tokens_number = vec![Token {
    kind: Kind::Number,
    keys: vec![Key::One, Key::Two, Key::Three, Key::Dot, Key::Four, Key::Five],
  }];

  assert_eq!(tokens, expected_tokens);
  assert_eq!(tokens_number, expected_tokens_number);
}

#[test]
#[should_panic = "Number should not contains more than one ."]
fn testing_digits_into_number_should_not_accept_more_than_one_dot() {
  let formula = String::from("12.3.45");
  digits_into_number(tokenize(formula).unwrap());
}

#[test]
#[should_panic = "Number should not start with ."]
fn testing_digits_into_number_should_not_start_with_dot() {
  let formula = String::from(".45");
  digits_into_number(tokenize(formula).unwrap());
}

#[test]
#[should_panic = "Number should not end with ."]
fn testing_digits_into_number_should_not_end_with_dot() {
  let formula = String::from("007.");
  digits_into_number(tokenize(formula).unwrap());
}

#[test]
fn testing_tokenize_char() {
  let formula = String::from("1+2");
  let token = tokenize_char(Kind::Operator, '+', &formula, 1).unwrap();

  let expected_token = Token {
    kind: Kind::Operator,
    keys: vec![Key::Addition],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_addition() {
  let formula = String::from("1+2");
  let token = tokenize_addition(&formula, 1).unwrap();

  let expected_token = Token {
    kind: Kind::Operator,
    keys: vec![Key::Addition],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_division() {
  let formula = String::from("1/2");
  let token = tokenize_division(&formula, 1).unwrap();

  let expected_token = Token {
    kind: Kind::Operator,
    keys: vec![Key::Division],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_multiplication() {
  let formula = String::from("1*2");
  let token = tokenize_multiplication(&formula, 1).unwrap();

  let expected_token = Token {
    kind: Kind::Operator,
    keys: vec![Key::Multiplication],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_subtration() {
  let formula = String::from("1-2");
  let token = tokenize_subtration(&formula, 1).unwrap();

  let expected_token = Token {
    kind: Kind::Operator,
    keys: vec![Key::Subtraction],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_box_bracket_open() {
  let formula = String::from("[1]");
  let token = tokenize_box_bracket_open(&formula, 0).unwrap();

  let expected_token = Token {
    kind: Kind::Bracket,
    keys: vec![Key::BoxOpen],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_box_bracket_close() {
  let formula = String::from("[1]");
  let token = tokenize_box_bracket_close(&formula, 2).unwrap();

  let expected_token = Token {
    kind: Kind::Bracket,
    keys: vec![Key::BoxClose],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_curly_bracket_open() {
  let formula = String::from("{2}");
  let token = tokenize_curly_bracket_open(&formula, 0).unwrap();

  let expected_token = Token {
    kind: Kind::Bracket,
    keys: vec![Key::CurlyOpen],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_curly_bracket_close() {
  let formula = String::from("{2}");
  let token = tokenize_curly_bracket_close(&formula, 2).unwrap();

  let expected_token = Token {
    kind: Kind::Bracket,
    keys: vec![Key::CurlyClose],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_round_bracket_open() {
  let formula = String::from("(3)");
  let token = tokenize_round_bracket_open(&formula, 0).unwrap();

  let expected_token = Token {
    kind: Kind::Bracket,
    keys: vec![Key::RoundOpen],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_round_bracket_close() {
  let formula = String::from("(3)");
  let token = tokenize_round_bracket_close(&formula, 2).unwrap();

  let expected_token = Token {
    kind: Kind::Bracket,
    keys: vec![Key::RoundClose],
  };

  assert_eq!(token, expected_token);
}

#[test]
#[should_panic = "Invalid kind"]
fn testing_tokenize_bracket() {
  let formula = String::from("(3)");
  tokenize_bracket(&formula, 2, Key::Multiplication, '(').unwrap();
}

#[test]
fn testing_tokenize_space() {
  let formula = String::from("1  +  2");
  let token = tokenize_space(&formula, 1).unwrap();

  let expected_token = Token {
    kind: Kind::Space,
    keys: vec![Key::Space],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_dot() {
  let formula = String::from("10.5");
  let token = tokenize_dot(&formula, 2).unwrap();

  let expected_token = Token {
    kind: Kind::Dot,
    keys: vec![Key::Dot],
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_decimal_digit() {
  let formula = String::from("0123456789");
  let token_zero = tokenize_decimal_digit(&formula, 0).unwrap();
  let token_one = tokenize_decimal_digit(&formula, 1).unwrap();
  let token_two = tokenize_decimal_digit(&formula, 2).unwrap();
  let token_three = tokenize_decimal_digit(&formula, 3).unwrap();
  let token_four = tokenize_decimal_digit(&formula, 4).unwrap();
  let token_five = tokenize_decimal_digit(&formula, 5).unwrap();
  let token_six = tokenize_decimal_digit(&formula, 6).unwrap();
  let token_seven = tokenize_decimal_digit(&formula, 7).unwrap();
  let token_eight = tokenize_decimal_digit(&formula, 8).unwrap();
  let token_nine = tokenize_decimal_digit(&formula, 9).unwrap();

  assert_eq!(
    token_zero,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Zero],
    }
  );
  assert_eq!(
    token_one,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::One],
    }
  );
  assert_eq!(
    token_two,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Two],
    }
  );
  assert_eq!(
    token_three,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Three],
    }
  );
  assert_eq!(
    token_four,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Four],
    }
  );
  assert_eq!(
    token_five,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Five],
    }
  );
  assert_eq!(
    token_six,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Six],
    }
  );
  assert_eq!(
    token_seven,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Seven],
    }
  );
  assert_eq!(
    token_eight,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Eight],
    }
  );
  assert_eq!(
    token_nine,
    Token {
      kind: Kind::Digit,
      keys: vec![Key::Nine],
    }
  );
}
