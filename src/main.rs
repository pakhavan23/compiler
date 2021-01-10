mod lexer;
mod parser;
mod parsing_table;
mod syntax_kinds;

use lexer::SyntaxToken;

fn main() {
    let mut tokens: Vec<SyntaxToken> = lexer::get_tokens(
        "Ashari id ,  o= 2 p ,t= 2^ \n agar{ 5 &MM 5 } \n [ \n e Jam 10^\n]   \nBegir(\"%d\",  X )^ \n Benevis(\"Hello World\")^ \n a = (((t Jam 4) Jam  YekiBala k  ) Zarb 7)^ \n ta { 5 &MM 7} [ \n U= 8^ \n ] ",
    );

    // for token in tokens {
    //     println!(
    //         "position: {} line: {} text: {} kind: {:?}",
    //         token.position, token.line, token.text, token.kind
    //     );
    // }
    let table = parsing_table::parsing_table();
    parser::parse(&mut tokens, table, true);
}
