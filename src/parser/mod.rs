use crate::tokenizer::{Key, Kind, Token};
use crate::Error;

#[derive(Debug, PartialEq)]
pub enum Node {
  Ast(Box<Ast>),
  Token(Token),
}

#[derive(Debug, PartialEq)]
pub struct Ast {
  pub operator: Option<Token>,
  pub operand_a: Option<Node>,
  pub operand_b: Option<Node>,
}
impl Ast {
  pub fn new(operator: Option<Token>, operand_a: Option<Node>, operand_b: Option<Node>) -> Ast {
    Ast {
      operator,
      operand_a,
      operand_b,
    }
  }
  pub fn new_operator(key: Key) -> Option<Token> {
    let token = Token::new_operator(key);
    match token.kind {
      Kind::Operator => Some(token),
      _ => None,
    }
  }
  pub fn new_number(keys: Vec<Key>) -> Option<Node> {
    let token = Token::new_number(keys);
    match token.kind {
      Kind::Number => Some(Node::Token(token)),
      _ => None,
    }
  }

  fn node_ast(ast: Ast) -> Option<Node> {
    Some(Node::Ast(Box::new(ast)))
  }
  fn node_token(token: Token) -> Option<Node> {
    match token.kind {
      Kind::Number => Some(Node::Token(token)),
      _ => None,
    }
  }
  fn node_operator(token: Token) -> Option<Token> {
    match token.kind {
      Kind::Operator => Some(token),
      _ => None,
    }
  }
  fn node_empty() -> Option<Node> {
    Option::None
  }
  fn is_empty(&self) -> bool {
    self.operator.is_none() && self.operand_a.is_none() && self.operand_b.is_none()
  }
}

pub fn parse(tokens: Result<Vec<Token>, Error>) -> Result<Ast, Error> {
  let tokens = remove_space(tokens?);
  if tokens.len() == 0 {
    return Err(Error::InvalidExpression("Input is empty".to_string()));
  }
  let ast = Ast::new(None, None, None);

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
        operator: Ast::new_operator(Key::Addition),
        operand_a: Ast::new_number(vec![Key::Zero]),
        operand_b: Ast::node_token(token),
      }),
      _ => Err(Error::InvalidExpression(
        "Bad format: Unique value should be a number".to_string(),
      )),
    };
  }

  if start_with_sign(&tokens) && ast.is_empty() {
    let mut new_tokens = vec![Token::new_number(vec![Key::Zero])];
    new_tokens.append(&mut tokens);
    return to_ast(Ok(new_tokens), ast);
  }

  let token = tokens.first().unwrap();

  match token.kind {
    Kind::Bracket => resolve_brackets(tokens, ast),
    Kind::Number => resolve_number(tokens, ast),
    Kind::Operator => resolve_operator(tokens, ast),
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

fn resolve_brackets(tokens: Vec<Token>, mut ast: Ast) -> Result<Ast, Error> {
  let bracket_expr = bracket_expression(tokens.clone())?;
  let rest_expression = tokens[bracket_expr.len()..].to_vec();
  let expression = remove_border_bracket(bracket_expr);
  let ast_expression = to_ast(Ok(expression), Ast::new(None, None, None))?;

  if rest_expression == [] && ast.is_empty() {
    return Ok(ast_expression);
  }

  match ast {
    Ast {
      operand_a: None,
      operand_b: None,
      ..
    } => ast.operand_a = Ast::node_ast(ast_expression),
    Ast {
      operand_a: Some(_),
      operand_b: None,
      ..
    } => ast.operand_b = Ast::node_ast(ast_expression),
    _ => {
      return Err(Error::InvalidExpression(
        "Bad format: There is no operator associated with this expression".to_string(),
      ))
    }
  }

  if rest_expression == [] {
    return Ok(ast);
  }

  to_ast(Ok(rest_expression), ast)
}

fn resolve_number(mut tokens: Vec<Token>, mut ast: Ast) -> Result<Ast, Error> {
  match ast {
    Ast {
      operand_a: None,
      operand_b: None,
      ..
    } => {
      let token = tokens.remove(0);
      ast.operand_a = Ast::node_token(token);
      to_ast(Ok(tokens), ast)
    }
    Ast {
      operand_a: Some(_),
      operand_b: None,
      ..
    } => {
      let token = tokens.remove(0);
      ast.operand_b = Ast::node_token(token);
      to_ast(Ok(tokens), ast)
    }
    _ => Err(Error::InvalidTokenSequence),
  }
}

fn resolve_operator(mut tokens: Vec<Token>, mut ast: Ast) -> Result<Ast, Error> {
  let token = tokens.remove(0);
  if token.kind != Kind::Operator {
    return Err(Error::InvalidTokenSequence);
  }

  if ast.operator == None {
    ast.operator = Ast::node_operator(token);
    return to_ast(Ok(tokens), ast);
  }

  let Ast {
    operator,
    operand_a,
    operand_b,
  } = ast;
  let operator = operator.unwrap();
  let curr_operator = &token.keys[0];
  let prev_operator = &operator.keys[0];

  if Key::precede(curr_operator, prev_operator) {
    let node_ast = Ast {
      operator: Ast::node_operator(token),
      operand_a: operand_b,
      operand_b: Ast::node_empty(),
    };
    let operand_b = to_ast(Ok(tokens), node_ast)?;

    let new_ast = Ast {
      operator: Ast::node_operator(operator),
      operand_a: operand_a,
      operand_b: Ast::node_ast(operand_b),
    };

    Ok(new_ast)
  } else {
    let operand_a = Ast {
      operator: Ast::node_operator(operator),
      operand_a,
      operand_b,
    };
    let new_ast = Ast {
      operator: Ast::node_operator(token),
      operand_a: Ast::node_ast(operand_a),
      operand_b: Ast::node_empty(),
    };

    to_ast(Ok(tokens), new_ast)
  }
}

fn bracket_expression(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
  if tokens[0].kind != Kind::Bracket {
    return Err(Error::InvalidExpression(
      "Bad format: Is expected a bracket at this point".to_string(),
    ));
  }
  let mut count_opens = 0;
  let mut count_closes = 0;
  let mut expression: Vec<Token> = vec![];
  let open = tokens[0].clone();
  let close = bracket_close_of(&open).unwrap();

  for token in tokens {
    match &token {
      token if *token == open => count_opens += 1,
      token if *token == close => count_closes += 1,
      _ => (),
    }
    expression.push(token);
    if count_opens > 0 && count_opens == count_closes {
      break;
    }
  }

  Ok(expression)
}

fn bracket_close_of(bracket: &Token) -> Option<Token> {
  match bracket.keys[0] {
    Key::CurlyOpen => Some(Token::new_bracket(Key::CurlyClose)),
    Key::BoxOpen => Some(Token::new_bracket(Key::BoxClose)),
    Key::RoundOpen => Some(Token::new_bracket(Key::RoundClose)),
    _ => None,
  }
}

fn remove_border_bracket(mut tokens: Vec<Token>) -> Vec<Token> {
  tokens.remove(0);
  tokens.remove(tokens.len() - 1);
  tokens
}

#[cfg(test)]
mod parser_spec;
