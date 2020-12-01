mod lexer;
mod syntax_kinds;

use lexer::SyntaxToken;

fn main() {
  let tokens: Vec<SyntaxToken> = lexer::get_tokens("Begir(\"sddf\", ali)^\nagar { 2 &MM 3 } [ 'AB' 'A' ]");
  for token in tokens {
      println!("position: {} line: {} text: {}", token.position, token.line, token.text);
  }
}
