mod lexer;
mod parser;

use lexer::SyntaxToken;
use parser::parse;

fn main() {
    let tokens: Vec<SyntaxToken> = lexer::get_tokens("x = 8 Zarb ( o Jam 2 )^");

    // (( o Jam 45 Zarb 2) Zarb 3 ) || x = 3 Jam 0.32^ || x = 3 Tagsim 5 Jam 0.32^ || x = ((78 Jam p) Zarb 55 Jam 50) Tagsim 10^
    // for token in tokens {
    //     println!("{:?}", token.kind);
    //     println!("Token = {}", token.text);
    // }
    parse(tokens);
}
