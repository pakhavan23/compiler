mod c_convertor;
mod lexer;
mod parser;
mod parsing_table;
mod semantics;
mod syntax_kinds;

use lexer::SyntaxToken;
use std::fs;
use std::io::Write;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{}", args[2]);
  let content =
    fs::read_to_string(args[1].to_string()).expect("Something went wrong reading the file");

  let mut tokens: Vec<SyntaxToken> = lexer::get_tokens(&content);

  let table = parsing_table::parsing_table();
  let next: bool = parser::parse(&mut tokens, table, true);
  if next {
    semantics::symbol_tab_filler(&mut tokens);
    let p = String::from("run.c");
    let mut s = String::from(&args[2]);
    s.push_str(&p);
    let data_path = std::path::Path::new(&s);
    let mut file = std::fs::File::create(data_path).expect("create file failed");
    file
      .write_all(c_convertor::convert_to_c(tokens).as_bytes())
      .expect("write failed");
  //std::io::stdin().read_line(&mut String::new()).unwrap();
  } else {
    //std::io::stdin().read_line(&mut String::new()).unwrap();
  }
}
