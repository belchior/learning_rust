use super::*;

#[test]
fn testing_tokenize() {
  let formula = String::from("-*/+()[]{} ");

  let tokens = tokenize(formula).unwrap();

  let expected_tokens = vec![
    Token {
      token_type: TokenType::Operator(Operator::Subtration),
      char_value: String::from('-'),
    },
    Token {
      token_type: TokenType::Operator(Operator::Multiplication),
      char_value: String::from('*'),
    },
    Token {
      token_type: TokenType::Operator(Operator::Division),
      char_value: String::from('/'),
    },
    Token {
      token_type: TokenType::Operator(Operator::Addition),
      char_value: String::from('+'),
    },
    Token {
      token_type: TokenType::Bracket(Bracket::Round),
      char_value: String::from('('),
    },
    Token {
      token_type: TokenType::Bracket(Bracket::Round),
      char_value: String::from(')'),
    },
    Token {
      token_type: TokenType::Bracket(Bracket::Box),
      char_value: String::from('['),
    },
    Token {
      token_type: TokenType::Bracket(Bracket::Box),
      char_value: String::from(']'),
    },
    Token {
      token_type: TokenType::Bracket(Bracket::Curly),
      char_value: String::from('{'),
    },
    Token {
      token_type: TokenType::Bracket(Bracket::Curly),
      char_value: String::from('}'),
    },
    Token {
      token_type: TokenType::Space,
      char_value: String::from(' '),
    },
  ];

  assert_eq!(tokens, expected_tokens);
}

#[test]
fn testing_join_spaces() {
  let tokens = tokenize(String::from("   ")).unwrap();
  let tokens_with_joined_space = join_spaces(tokenize(String::from("   ")).unwrap());

  let expected_tokens = vec![
    Token {
      token_type: TokenType::Space,
      char_value: String::from(' '),
    },
    Token {
      token_type: TokenType::Space,
      char_value: String::from(' '),
    },
    Token {
      token_type: TokenType::Space,
      char_value: String::from(' '),
    },
  ];

  let expected_tokens_with_joined_space = vec![Token {
    token_type: TokenType::Space,
    char_value: String::from("   "),
  }];

  assert_eq!(tokens, expected_tokens);
  assert_eq!(tokens_with_joined_space, expected_tokens_with_joined_space);
}

#[test]
fn testing_tokenize_char() {
  let formula = String::from("1+2");
  let token = tokenize_char(TokenType::Operator(Operator::Addition), '+', &formula, 1).unwrap();

  let expected_token = Token {
    token_type: TokenType::Operator(Operator::Addition),
    char_value: String::from('+'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_addition() {
  let formula = String::from("1+2");
  let token = tokenize_addition(&formula, 1).unwrap();

  let expected_token = Token {
    token_type: TokenType::Operator(Operator::Addition),
    char_value: String::from('+'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_division() {
  let formula = String::from("1/2");
  let token = tokenize_division(&formula, 1).unwrap();

  let expected_token = Token {
    token_type: TokenType::Operator(Operator::Division),
    char_value: String::from('/'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_multiplication() {
  let formula = String::from("1*2");
  let token = tokenize_multiplication(&formula, 1).unwrap();

  let expected_token = Token {
    token_type: TokenType::Operator(Operator::Multiplication),
    char_value: String::from('*'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_subtration() {
  let formula = String::from("1-2");
  let token = tokenize_subtration(&formula, 1).unwrap();

  let expected_token = Token {
    token_type: TokenType::Operator(Operator::Subtration),
    char_value: String::from('-'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_box_bracket_open() {
  let formula = String::from("[1]");
  let token = tokenize_box_bracket_open(&formula, 0).unwrap();

  let expected_token = Token {
    token_type: TokenType::Bracket(Bracket::Box),
    char_value: String::from('['),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_box_bracket_close() {
  let formula = String::from("[1]");
  let token = tokenize_box_bracket_close(&formula, 2).unwrap();

  let expected_token = Token {
    token_type: TokenType::Bracket(Bracket::Box),
    char_value: String::from(']'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_curly_bracket_open() {
  let formula = String::from("{2}");
  let token = tokenize_curly_bracket_open(&formula, 0).unwrap();

  let expected_token = Token {
    token_type: TokenType::Bracket(Bracket::Curly),
    char_value: String::from('{'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_curly_bracket_close() {
  let formula = String::from("{2}");
  let token = tokenize_curly_bracket_close(&formula, 2).unwrap();

  let expected_token = Token {
    token_type: TokenType::Bracket(Bracket::Curly),
    char_value: String::from('}'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_round_bracket_open() {
  let formula = String::from("(3)");
  let token = tokenize_round_bracket_open(&formula, 0).unwrap();

  let expected_token = Token {
    token_type: TokenType::Bracket(Bracket::Round),
    char_value: String::from('('),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_round_bracket_close() {
  let formula = String::from("(3)");
  let token = tokenize_round_bracket_close(&formula, 2).unwrap();

  let expected_token = Token {
    token_type: TokenType::Bracket(Bracket::Round),
    char_value: String::from(')'),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_space() {
  let formula = String::from("1  +  2");
  let token = tokenize_space(&formula, 1).unwrap();

  let expected_token = Token {
    token_type: TokenType::Space,
    char_value: String::from(' '),
  };

  assert_eq!(token, expected_token);
}

#[test]
fn testing_tokenize_dot() {
  let formula = String::from("10.5");
  let token = tokenize_dot(&formula, 2).unwrap();

  let expected_token = Token {
    token_type: TokenType::Operand(Operand::Dot),
    char_value: String::from('.'),
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
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('0'),
    }
  );
  assert_eq!(
    token_one,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('1'),
    }
  );
  assert_eq!(
    token_two,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('2'),
    }
  );
  assert_eq!(
    token_three,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('3'),
    }
  );
  assert_eq!(
    token_four,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('4'),
    }
  );
  assert_eq!(
    token_five,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('5'),
    }
  );
  assert_eq!(
    token_six,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('6'),
    }
  );
  assert_eq!(
    token_seven,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('7'),
    }
  );
  assert_eq!(
    token_eight,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('8'),
    }
  );
  assert_eq!(
    token_nine,
    Token {
      token_type: TokenType::Operand(Operand::Digit),
      char_value: String::from('9'),
    }
  );
}
