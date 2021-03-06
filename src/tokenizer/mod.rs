use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
  Bracket,
  Operator,
  Number,
  Digit,
  Dot,
  Space,
}
impl Kind {
  fn keys(&self) -> Vec<Key> {
    use Key::*;
    match *self {
      Kind::Bracket => vec![RoundOpen, RoundClose, BoxOpen, BoxClose, CurlyOpen, CurlyClose],
      Kind::Operator => vec![Addition, Subtraction, Multiplication, Division],
      Kind::Number => vec![Dot, Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine],
      Kind::Digit => vec![Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine],
      Kind::Dot => vec![Dot],
      Kind::Space => vec![Space],
    }
  }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Key {
  // Kind::Bracket
  RoundOpen,  // '(',
  RoundClose, // ')',
  BoxOpen,    // '[',
  BoxClose,   // ']',
  CurlyOpen,  // '{',
  CurlyClose, // '}',

  // Kind::Operator
  Multiplication, // '*',
  Division,       // '/',
  Addition,       // '+',
  Subtraction,    // '-',

  // Kind::Digit
  Zero,  // '0',
  One,   // '1',
  Two,   // '2',
  Three, // '3',
  Four,  // '4',
  Five,  // '5',
  Six,   // '6',
  Seven, // '7',
  Eight, // '8',
  Nine,  // '9',

  // Kind::Dot
  Dot, // '.',

  // Kind::Space
  Space, // ' ',
}
impl Key {
  fn into_key(char_value: char) -> Key {
    use Key::*;
    match char_value {
      '(' => RoundOpen,
      ')' => RoundClose,
      '[' => BoxOpen,
      ']' => BoxClose,
      '{' => CurlyOpen,
      '}' => CurlyClose,
      '*' => Multiplication,
      '/' => Division,
      '+' => Addition,
      '-' => Subtraction,
      '0' => Zero,
      '1' => One,
      '2' => Two,
      '3' => Three,
      '4' => Four,
      '5' => Five,
      '6' => Six,
      '7' => Seven,
      '8' => Eight,
      '9' => Nine,
      '.' => Dot,
      ' ' => Space,
      _ => panic!(format!("Undefined char: {}", char_value)),
    }
  }

  fn kind(&self) -> Kind {
    use Key::*;
    match *self {
      BoxOpen | BoxClose | CurlyOpen | CurlyClose | RoundOpen | RoundClose => Kind::Bracket,
      Addition | Subtraction | Multiplication | Division => Kind::Operator,
      Zero | One | Two | Three | Four | Five | Six | Seven | Eight | Nine => Kind::Digit,
      Dot => Kind::Dot,
      Space => Kind::Space,
    }
  }

  pub fn precede(key_a: &Key, key_b: &Key) -> bool {
    match (key_a, key_b) {
      (Key::Multiplication, Key::Division) => false,
      (Key::Division, Key::Multiplication) => false,
      (Key::Addition, Key::Subtraction) => false,
      (Key::Subtraction, Key::Addition) => false,
      _ => key_a < key_b,
    }
  }

  fn to_char(&self) -> char {
    use Key::*;
    match self {
      RoundOpen => '(',
      RoundClose => ')',
      BoxOpen => '[',
      BoxClose => ']',
      CurlyOpen => '{',
      CurlyClose => '}',
      Multiplication => '*',
      Division => '/',
      Addition => '+',
      Subtraction => '-',
      Zero => '0',
      One => '1',
      Two => '2',
      Three => '3',
      Four => '4',
      Five => '5',
      Six => '6',
      Seven => '7',
      Eight => '8',
      Nine => '9',
      Dot => '.',
      Space => ' ',
    }
  }

  pub fn to_string(&self) -> String {
    self.to_char().to_string()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
  pub kind: Kind,
  pub keys: Vec<Key>,
}
impl Token {
  pub fn new(kind: Kind, key: Key) -> Token {
    if key.kind() != kind {
      panic!("Invalid token arguments");
    }
    Token { kind, keys: vec![key] }
  }

  pub fn new_bracket(key: Key) -> Token {
    if key.kind() != Kind::Bracket {
      panic!("Invalid token arguments")
    }
    Self::new(Kind::Bracket, key)
  }

  pub fn new_number(keys: Vec<Key>) -> Token {
    if keys.len() == 0 {
      panic!("Invalid token arguments")
    }

    {
      let valid_keys = Kind::Number.keys();
      let is_valid_keys = keys
        .iter()
        .fold(true, |acc, key| acc == true && valid_keys.contains(&key));

      if is_valid_keys == false {
        panic!("Invalid token arguments")
      }
    }

    {
      let mut is_valid_sequence = true;

      if keys.starts_with(&[Key::Dot]) || keys.ends_with(&[Key::Dot]) {
        is_valid_sequence = false;
      }

      if keys.iter().filter(|&key| key.kind() == Kind::Dot).count() > 1 {
        is_valid_sequence = false;
      }

      if is_valid_sequence == false {
        panic!("Invalid token arguments")
      }
    }

    Token {
      kind: Kind::Number,
      keys,
    }
  }

  pub fn new_operator(key: Key) -> Token {
    Self::new(Kind::Operator, key)
  }

  #[allow(dead_code)]
  pub fn new_space(keys: Vec<Key>) -> Token {
    if keys.len() == 0 {
      panic!("Invalid token arguments")
    }

    {
      let is_valid_keys = keys.iter().filter(|key| key.kind() != Kind::Space).count() == 0;

      if is_valid_keys == false {
        panic!("Invalid token arguments")
      }
    }

    Token {
      kind: Kind::Space,
      keys,
    }
  }

  pub fn to_string(&self) -> String {
    self.keys.iter().fold("".to_string(), |mut acc, key| {
      acc.push_str(&key.to_string());
      acc
    })
  }
}

pub fn tokenize(formula: String) -> Result<Vec<Token>, Error> {
  let tokens = Vec::new();
  let current: usize = 0;
  run_tokenize(formula, tokens, current)
    .map(digits_into_number)
    .map(join_spaces)
}

fn run_tokenize(formula: String, mut tokens: Vec<Token>, mut current: usize) -> Result<Vec<Token>, Error> {
  if current >= formula.len() {
    return Ok(tokens);
  }

  let mut new_tokens: Vec<Token> = Vec::new();

  for tokenizer in &TOKENIZERS {
    let token = tokenizer(&formula, current);
    if let Some(token) = token {
      current += token.to_string().len();
      new_tokens.push(token);
    }
  }

  if new_tokens.len() == 0 {
    let err_message = format!("Error: Undefined symbol start {}", formula.split_at(current).1);
    return Err(Error::UndefinedSymbol(err_message));
  }

  tokens.append(&mut new_tokens);

  run_tokenize(formula, tokens, current)
}

fn join_spaces(tokens: Vec<Token>) -> Vec<Token> {
  let mut token_list: Vec<Token> = Vec::new();
  let mut token_space: Option<Token> = None;

  for token in tokens {
    match token.kind {
      Kind::Space => {
        token_space = Some(Token {
          kind: Kind::Space,
          keys: match token_space {
            None => token.keys,
            Some(prev_token) => vec![prev_token.keys, token.keys].concat(),
          },
        });
      }

      _ => {
        if let Some(prev_token) = token_space {
          token_list.push(prev_token);
          token_space = None;
        }
        token_list.push(token);
      }
    }
  }

  if let Some(prev_token) = token_space {
    token_list.push(prev_token);
  }

  token_list
}

fn digits_into_number(tokens: Vec<Token>) -> Vec<Token> {
  let mut token_list: Vec<Token> = Vec::new();
  let mut token_number: Option<Token> = None;

  for token in tokens {
    match token.kind {
      Kind::Digit => {
        token_number = Some(Token {
          kind: Kind::Number,
          keys: match token_number {
            None => token.keys,
            Some(prev_token) => vec![prev_token.keys, token.keys].concat(),
          },
        })
      }

      Kind::Dot => {
        token_number = match token_number {
          None => panic!("Number should not start with ."),
          Some(prev_token) if prev_token.keys.contains(&Key::Dot) => {
            panic!("Number should not contains more than one .")
          }
          Some(prev_token) => Some(Token {
            kind: Kind::Number,
            keys: vec![prev_token.keys, token.keys].concat(),
          }),
        }
      }

      _ => {
        if let Some(prev_token) = token_number {
          token_list.push(prev_token);
          token_number = None;
        }
        token_list.push(token);
      }
    }
  }

  if let Some(prev_token) = token_number {
    token_list.push(prev_token);
  }

  for token in &token_list {
    if token.kind == Kind::Number && token.keys.ends_with(&[Key::Dot]) {
      panic!("Number should not end with .")
    }
  }

  token_list
}

static TOKENIZERS: [fn(formula: &String, current: usize) -> Option<Token>; 13] = [
  tokenize_addition,
  tokenize_box_bracket_close,
  tokenize_box_bracket_open,
  tokenize_curly_bracket_close,
  tokenize_curly_bracket_open,
  tokenize_decimal_digit,
  tokenize_division,
  tokenize_dot,
  tokenize_multiplication,
  tokenize_round_bracket_close,
  tokenize_round_bracket_open,
  tokenize_space,
  tokenize_subtration,
];

fn tokenize_addition(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(Kind::Operator, '+', &formula, current)
}

fn tokenize_division(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(Kind::Operator, '/', formula, current)
}

fn tokenize_multiplication(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(Kind::Operator, '*', formula, current)
}

fn tokenize_subtration(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(Kind::Operator, '-', formula, current)
}

fn tokenize_box_bracket_open(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Key::BoxOpen, '[')
}

fn tokenize_box_bracket_close(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Key::BoxClose, ']')
}

fn tokenize_curly_bracket_open(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Key::CurlyOpen, '{')
}

fn tokenize_curly_bracket_close(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Key::CurlyClose, '}')
}

fn tokenize_round_bracket_open(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Key::RoundOpen, '(')
}

fn tokenize_round_bracket_close(formula: &String, current: usize) -> Option<Token> {
  tokenize_bracket(formula, current, Key::RoundClose, ')')
}

fn tokenize_space(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(Kind::Space, ' ', formula, current)
}

fn tokenize_dot(formula: &String, current: usize) -> Option<Token> {
  tokenize_char(Kind::Dot, '.', formula, current)
}

fn tokenize_char(kind: Kind, char_value: char, formula: &String, current: usize) -> Option<Token> {
  match formula.chars().nth(current) {
    Some(value) if value == char_value => Some(Token {
      kind,
      keys: vec![Key::into_key(char_value)],
    }),
    Some(_) => None,
    None => None,
  }
}

fn tokenize_bracket(formula: &String, current: usize, bracket: Key, char_value: char) -> Option<Token> {
  match bracket.kind() {
    Kind::Bracket => tokenize_char(Kind::Bracket, char_value, formula, current),
    _ => panic!("Invalid kind"),
  }
}

fn tokenize_decimal_digit(formula: &String, current: usize) -> Option<Token> {
  let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
  let value = formula.chars().nth(current);

  digits.iter().find(|&&digit| Some(digit) == value).map(|&digit| Token {
    kind: Kind::Digit,
    keys: vec![Key::into_key(digit)],
  })
}

#[cfg(test)]
mod tokenizer_spec;
