mod error;
mod parser;
mod tokenizer;

use error::Error;
use std::env;

fn main() {
  let formula = env::args().nth(1);

  match formula {
    Some(formula) => {
      let token_list = Calc::calculate(formula);
      println!("{:?}", token_list);
    }
    None => eprintln!("Error: Formula is required\nUse: calc '1+2-3*4/5'"),
  }
}

struct Calc;

impl Calc {
  fn calculate(formula: String) -> Result<Vec<tokenizer::Token>, Error> {
    tokenizer::tokenize(formula)
  }
}
