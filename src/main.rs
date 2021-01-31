mod lexer;
mod syntax_kinds;
mod c_convertor;

use lexer::SyntaxToken;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
  let contents = fs::read_to_string("content.txt").expect("Something went wrong reading the file");
  let tokens: Vec<SyntaxToken> = lexer::get_tokens(&contents);
  let mut file = std::fs::File::create("run.c").expect("create file failed");
  file.write_all(c_convertor::convert_to_c(tokens).as_bytes()).expect("write failed");
}
