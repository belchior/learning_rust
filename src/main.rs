use std::env;

mod tokenizer;

fn main() {
  let mut args = env::args();
  args.next();
  let formula = args.next();

  match formula {
    Some(formula) => {
      let token_list = Calc::calculate(formula);
      println!("{:?}", token_list);
    }
    None => eprintln!("Error: Formula is required\nUse: calc '1+2-3*4/5'"),
  }
}

pub struct Calc;

impl Calc {
  pub fn calculate(formula: String) -> Result<Vec<tokenizer::Token>, String> {
    tokenizer::tokenize(formula)
  }
}
