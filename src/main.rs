mod lexer;
mod parser;

use lexer::SyntaxToken;
use parser::parse;

fn main() {
    let tokens: Vec<SyntaxToken> =
        lexer::get_tokens("x = (( o Jam 45 Zarb 2) Zarb 3 ) Tagsim 5 Jam 0.32^");

    // (( o Jam 45 Zarb 2) Zarb 3 ) || x = 3 Jam 0.32^ || x = 3 Tagsim 5 Jam 0.32^
    // for token in tokens {
    //     println!("{:?}", token.kind);
    //     println!("Token = {}", token.text);
    // }
    parse(tokens);
}
