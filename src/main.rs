mod lexer;

use lexer::SyntaxToken;

fn main() {
  // this is erfan\n im 20\n javascript []agar {} programmer\n and \"wanna\" be a rust developer []\nsdfasd
  let tokens: Vec<SyntaxToken> = lexer::get_tokens("Begir(\"sddf\", ali)^\nagar { 2 &MM 3 } [ ]");
  for token in tokens {
      println!("position: {} line: {} text: {}", token.position, token.line, token.text);
  }

  // let tokens = lexer::get_syntax("123&MM&B&BM132erfan".to_string(), 0, 1);

  // for token in tokens {
  //   println!("position: {} line: {} text: {}", token.position, token.line, token.text);
  // }
}
