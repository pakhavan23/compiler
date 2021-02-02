mod lexer;
mod parser;
mod parsing_table;
mod semantics;
mod syntax_kinds;
mod c_convertor;

use lexer::SyntaxToken;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
  let content = fs::read_to_string("content.txt").expect("Something went wrong reading the file");

  let mut tokens: Vec<SyntaxToken> = lexer::get_tokens(&content);

  let table = parsing_table::parsing_table();
  let next: bool = parser::parse(&mut tokens, table, true);
  if next {
    semantics::symbol_tab_filler(&mut tokens);
    let mut file = std::fs::File::create("run.c").expect("create file failed");
    file.write_all(c_convertor::convert_to_c(tokens).as_bytes()).expect("write failed");
    std::io::stdin().read_line(&mut String::new()).unwrap();
  } else {
    std::io::stdin().read_line(&mut String::new()).unwrap();
  }
}