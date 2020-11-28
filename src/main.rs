mod lexer;
mod parser;

use lexer::SyntaxToken;
use parser::parse;

fn main(){
    let tokens: Vec<SyntaxToken> = lexer::get_tokens(" Benevis(X)^");

    parse(tokens);
}