mod lexer;
mod parser;
mod parsing_table;
mod syntax_kinds;

use lexer::SyntaxToken;
use syntax_kinds::SyntaxKind;

fn main() {
    let mut tokens: Vec<SyntaxToken> = lexer::get_tokens(
        "Sahih a = 7 ,r  ^ \n agar { X &B 3 } [ e Jam 10 ^ ]    \n Begir(\"%d\",X )^ \n Benevis(\"Hello World)^",
    );
    // for token in tokens {
    //     println!(
    //         "position: {} line: {} text: {} kind: {:?}",
    //         token.position, token.line, token.text, token.kind
    //     );
    // }
    let last_token = SyntaxToken {
        line: 0,
        text: "$".to_string(),
        kind: SyntaxKind::DollaeSignToken,
        position: 0,
    };
    tokens.push(last_token);
    let table = parsing_table::parsing_table();
    parser::parse(tokens, table);
}
