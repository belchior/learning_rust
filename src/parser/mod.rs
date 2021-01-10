#![allow(dead_code)]

use crate::Error;

#[derive(Debug, PartialEq)]
pub enum Kind {
  Bracket,
  Digit,
  Operand,
  Operator,
  Space,
}

#[derive(Debug, PartialEq)]
pub enum Token {
  Addition,          // '+',
  Subtraction,       // '-',
  Multiplication,    // '*',
  Division,          // '/',
  BracketRoundOpen,  // '(',
  BracketRoundClose, // ')',
  BracketBoxOpen,    // '[',
  BracketBoxClose,   // ']',
  BracketCurlyOpen,  // '{',
  BracketCurlyClose, // '}',
  Dot,               // '.',
  Zero,              // '0',
  One,               // '1',
  Two,               // '2',
  Three,             // '3',
  Four,              // '4',
  Five,              // '5',
  Six,               // '6',
  Seven,             // '7',
  Eight,             // '8',
  Nine,              // '9',
  Space(String),     // ' ',
  Number(String),    // '1.2',
}
impl Token {
  pub fn value(&self) -> String {
    use Token::*;
    match self {
      Addition => '+'.to_string(),
      Subtraction => '-'.to_string(),
      Multiplication => '*'.to_string(),
      Division => '/'.to_string(),
      BracketRoundOpen => '('.to_string(),
      BracketRoundClose => ')'.to_string(),
      BracketBoxOpen => '['.to_string(),
      BracketBoxClose => ']'.to_string(),
      BracketCurlyOpen => '{'.to_string(),
      BracketCurlyClose => '}'.to_string(),
      Dot => '.'.to_string(),
      Zero => '0'.to_string(),
      One => '1'.to_string(),
      Two => '2'.to_string(),
      Three => '3'.to_string(),
      Four => '4'.to_string(),
      Five => '5'.to_string(),
      Six => '6'.to_string(),
      Seven => '7'.to_string(),
      Eight => '8'.to_string(),
      Nine => '9'.to_string(),
      Space(value) => value.clone(),
      Number(value) => value.clone(),
    }
  }

  pub fn kind(&self) -> Kind {
    use Token::*;
    match *self {
      Space(_) => Kind::Space,
      Number(_) => Kind::Operand,
      Addition | Subtraction | Multiplication | Division => Kind::Operator,
      Zero | One | Two | Three | Four | Five | Six | Seven | Eight | Nine | Dot => Kind::Digit,
      BracketBoxOpen | BracketBoxClose | BracketCurlyOpen | BracketCurlyClose
      | BracketRoundOpen | BracketRoundClose => Kind::Bracket,
    }
  }
}

pub enum Node {
  Ast(Box<Ast>),
  Token(Token),
}

pub struct Ast {
  pub operator: Option<Token>,
  pub operand_a: Option<Node>,
  pub operand_b: Option<Node>,
}
impl Ast {
  fn ast(ast: Ast) -> Option<Node> {
    Some(Node::Ast(Box::new(ast)))
  }
  fn token(token: Token) -> Option<Node> {
    Some(Node::Token(token))
  }
  fn operator(token: Token) -> Option<Token> {
    Some(token)
  }
  fn empty_operator() -> Option<Token> {
    Option::None
  }
  fn empty() -> Option<Node> {
    Option::None
  }
  fn is_empty(&self) -> bool {
    self.operator.is_none() && self.operand_a.is_none() && self.operand_b.is_none()
  }
}

fn remove_space(tokens: Vec<Token>) -> Vec<Token> {
  tokens
    .into_iter()
    .filter(|token| token.kind() != Kind::Space)
    .collect()
}

pub fn parser(tokens: Result<Vec<Token>, Error>) -> Result<Ast, Error> {
  let tokens = remove_space(tokens?);
  if tokens.len() == 0 {
    return Err(Error::InvalidExpression);
  }
  let ast = Ast {
    operator: Ast::empty_operator(),
    operand_a: Ast::empty(),
    operand_b: Ast::empty(),
  };
  to_ast(Ok(tokens), ast)
}

fn to_ast(tokens: Result<Vec<Token>, Error>, ast: Ast) -> Result<Ast, Error> {
  let mut tokens = tokens?;
  let len = tokens.len();

  if len == 0 {
    return Ok(ast);
  }

  if len == 1 && ast.is_empty() {
    let token = tokens.pop().unwrap();

    return match token.kind() {
      Kind::Operand => Ok(Ast {
        operator: Ast::operator(Token::Addition),
        operand_a: Ast::token(Token::Zero),
        operand_b: Ast::token(token),
      }),
      _ => Err(Error::InvalidExpression),
    };
  }

  if start_with_sign(&tokens) && ast.is_empty() {
    let mut new_tokens = vec![Token::Zero];
    new_tokens.append(&mut tokens);
    return to_ast(Ok(new_tokens), ast);
  }

  Ok(ast)
}

fn start_with_sign(tokens: &Vec<Token>) -> bool {
  if tokens.len() == 0 {
    return false;
  }
  *&tokens[0] == Token::Addition || *&tokens[0] == Token::Subtraction
}

#[cfg(test)]
mod parser_spec;
