use super::*;

#[test]
fn testing_remove_space() {
  let tokens = vec![
    Token::One,
    Token::Space("  ".to_string()),
    Token::Addition,
    Token::Space("  ".to_string()),
    Token::Two,
  ];

  let expected_tokens = vec![Token::One, Token::Addition, Token::Two];

  let tokens_without_space = remove_space(tokens);
  assert_eq!(tokens_without_space, expected_tokens);
}
