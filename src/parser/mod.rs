#![allow(dead_code)]

use crate::tokenizer::{Key, Kind, Token};
use crate::Error;

#[derive(Debug, PartialEq)]
enum Node {
  Ast(Box<Ast>),
  Token(Token),
}

#[derive(Debug, PartialEq)]
struct Ast {
  operator: Option<Token>,
  operand_a: Option<Node>,
  operand_b: Option<Node>,
}
impl Ast {
  fn new() -> Ast {
    Ast {
      operator: Ast::empty_operator(),
      operand_a: Ast::empty(),
      operand_b: Ast::empty(),
    }
  }
  fn ast(ast: Ast) -> Option<Node> {
    Some(Node::Ast(Box::new(ast)))
  }
  fn token(token: Token) -> Option<Node> {
    match token.kind {
      Kind::Number => Some(Node::Token(token)),
      _ => None,
    }
  }
  fn operator(token: Token) -> Option<Token> {
    match token.kind {
      Kind::Operator => Some(token),
      _ => None,
    }
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

fn parser(tokens: Result<Vec<Token>, Error>) -> Result<Ast, Error> {
  let tokens = remove_space(tokens?);
  if tokens.len() == 0 {
    return Err(Error::InvalidExpression);
  }
  let ast = Ast::new();

  to_ast(Ok(tokens), ast)
}

fn to_ast(tokens: Result<Vec<Token>, Error>, ast: Ast) -> Result<Ast, Error> {
  let mut tokens = tokens?;
  let token_len = tokens.len();

  if token_len == 0 {
    return Ok(ast);
  }

  if token_len == 1 && ast.is_empty() {
    let token = tokens.pop().unwrap();

    return match token.kind {
      Kind::Number => Ok(Ast {
        operator: Ast::operator(Token::new_operator(Key::Addition)),
        operand_a: Ast::token(Token::new_number(vec![Key::Zero])),
        operand_b: Ast::token(token),
      }),
      _ => Err(Error::InvalidExpression),
    };
  }

  if start_with_sign(&tokens) && ast.is_empty() {
    let mut new_tokens = vec![Token::new_number(vec![Key::Zero])];
    new_tokens.append(&mut tokens);
    return to_ast(Ok(new_tokens), ast);
  }

  let token = tokens.first().unwrap();

  match token.kind {
    Kind::Operator => resolve_operator(tokens, ast),
    Kind::Number => resolve_number(tokens, ast),
    _ => Err(Error::InvalidTokenSequence),
  }
}

fn remove_space(tokens: Vec<Token>) -> Vec<Token> {
  tokens.into_iter().filter(|token| token.kind != Kind::Space).collect()
}

fn start_with_sign(tokens: &Vec<Token>) -> bool {
  if tokens.len() == 0 {
    return false;
  }
  let key = &tokens[0].keys[0];
  *key == Key::Addition || *key == Key::Subtraction
}

fn resolve_operator(mut tokens: Vec<Token>, mut ast: Ast) -> Result<Ast, Error> {
  match ast.operator {
    None => {
      let token = tokens.remove(0);
      ast.operator = Ast::operator(token);
      to_ast(Ok(tokens), ast)
    }
    Some(_) => panic!("Operador já existe"),
  }
}

fn resolve_number(mut tokens: Vec<Token>, mut ast: Ast) -> Result<Ast, Error> {
  match ast {
    Ast {
      operator: _,
      operand_a: None,
      operand_b: None,
    } => {
      let token = tokens.remove(0);
      ast.operand_a = Ast::token(token);
      to_ast(Ok(tokens), ast)
    }
    Ast {
      operator: _,
      operand_a: Some(_),
      operand_b: None,
    } => {
      let token = tokens.remove(0);
      ast.operand_b = Ast::token(token);
      to_ast(Ok(tokens), ast)
    }
    _ => {
      panic!("Ast cheia")
    }
  }
}

#[cfg(test)]
mod parser_spec;
