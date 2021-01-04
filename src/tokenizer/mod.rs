#[derive(PartialEq, Debug)]
pub struct Token {
  token_type: TokenType,
  char_value: String,
}

pub fn tokenize(formula: String) -> Result<Vec<Token>, String> {
  let tokens = Vec::new();
  let current: usize = 0;
  run_tokenize(formula, tokens, current)
}

fn run_tokenize(
  formula: String,
  mut tokens: Vec<Token>,
  mut current: usize,
) -> Result<Vec<Token>, String> {
  if current >= formula.len() {
    return Ok(tokens);
  }

  let mut new_tokens: Vec<Token> = Vec::new();

  for tokenizer in &TOKENIZERS {
    let token = tokenizer(&formula, current);
    if let Some(token) = token {
      current += token.char_value.len();
      new_tokens.push(token);
    }
  }

  if new_tokens.len() == 0 {
    let err_message = format!(
      "Error: Undefined symbol start {}",
      formula.split_at(current).1
    );
    return Err(err_message);
  }

  tokens.append(&mut new_tokens);

  run_tokenize(formula, tokens, current)
}

fn join_spaces(tokens: Vec<Token>) -> Vec<Token> {
  let mut token_list: Vec<Token> = Vec::new();
  let mut token_space: Option<Token> = None;

  for token in tokens {
    if token.token_type == TokenType::Space {
      token_space = Some(Token {
        token_type: TokenType::Space,
        char_value: if let Some(prev_token) = token_space {
          format!("{}{}", prev_token.char_value, token.char_value)
        } else {
          token.char_value
        },
      });
      continue;
    }
    if let Some(prev_token) = token_space {
      token_list.push(prev_token);
      token_space = None;
    }
    token_list.push(token);
  }

  if let Some(prev_token) = token_space {
    token_list.push(prev_token);
  }

  token_list
}

static TOKENIZERS: [fn(formula: &String, current: usize) -> Option<Token>; 11] = [
  tokenize_addition,
  tokenize_division,
  tokenize_multiplication,
  tokenize_subtration,
  tokenize_box_bracket_open,
  tokenize_box_bracket_close,
  tokenize_curly_bracket_open,
  tokenize_curly_bracket_close,
  tokenize_round_bracket_open,
  tokenize_round_bracket_close,
  tokenize_space,
];

fn tokenize_addition(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(
    TokenType::Operator(Operator::Addition),
    '+',
    &formula,
    current,
  )
}

fn tokenize_division(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(
    TokenType::Operator(Operator::Division),
    '/',
    formula,
    current,
  )
}

fn tokenize_multiplication(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(
    TokenType::Operator(Operator::Multiplication),
    '*',
    formula,
    current,
  )
}

fn tokenize_subtration(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(
    TokenType::Operator(Operator::Subtration),
    '-',
    formula,
    current,
  )
}

fn tokenize_box_bracket_open(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Bracket::Box, '[')
}

fn tokenize_box_bracket_close(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Bracket::Box, ']')
}

fn tokenize_curly_bracket_open(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Bracket::Curly, '{')
}

fn tokenize_curly_bracket_close(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Bracket::Curly, '}')
}

fn tokenize_round_bracket_open(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Bracket::Round, '(')
}

fn tokenize_round_bracket_close(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Bracket::Round, ')')
}

fn tokenize_space(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(TokenType::Space, ' ', formula, current)
}

fn tokenize_dot(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(TokenType::Operand(Operand::Dot), '.', formula, current)
}

fn tokenize_decimal_digit(formula: &String, current: usize) -> Option<Token> {
  let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
  let value = formula.chars().nth(current);

  for digit in digits.iter() {
    if value == Some(*digit) {
      return Some(Token {
        token_type: TokenType::Operand(Operand::Digit),
        char_value: String::from(*digit),
      });
    }
  }
  None
}

fn tokenize_bracket(
  formula: &String,
  current: usize,
  bracket: Bracket,
  char_value: char,
) -> Option<Token> {
  tokenize_char(TokenType::Bracket(bracket), char_value, formula, current)
}

fn tokenize_char(
  token_type: TokenType,
  char_value: char,
  formula: &String,
  current: usize,
) -> Option<Token> {
  if formula.chars().nth(current) == Some(char_value) {
    return Some(Token {
      token_type,
      char_value: String::from(char_value),
    });
  }
  None
}

#[derive(PartialEq, Debug)]
enum Operator {
  Addition,
  Division,
  Multiplication,
  Subtration,
}

#[derive(PartialEq, Debug)]
enum Operand {
  Digit,
  Dot,
  Number,
}

#[derive(PartialEq, Debug)]
enum Bracket {
  Box,
  Curly,
  Round,
}

#[derive(PartialEq, Debug)]
enum TokenType {
  Bracket(Bracket),
  Operand(Operand),
  Operator(Operator),
  Space,
}

#[cfg(test)]
mod tokenizer_spec;
