mod error;
mod interpreter;
mod parser;
mod tokenizer;

use error::Error;
use std::env;

fn main() {
  let formula = env::args().nth(1);

  match formula {
    Some(formula) => {
      let result = Calc::calculate(formula).unwrap();
      println!("{}", result);
    }
    None => eprintln!("Error: Formula is required\nUse: calc '1+2-3*4/5'"),
  }
}

struct Calc;

impl Calc {
  fn calculate(formula: String) -> Result<String, Error> {
    interpreter::run(formula)
  }
}

#[cfg(test)]
mod main_spec;
