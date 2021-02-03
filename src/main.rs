mod lexer;
mod parser;
mod parsing_table;
mod semantics;
mod syntax_kinds;

use lexer::SyntaxToken;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut tokens: Vec<SyntaxToken> = lexer::get_tokens("Sahih x^\n Begir(\"%d\",&x)^ ");
    // let mut tokens: Vec<SyntaxToken> = lexer::get_tokens(
    // "Ashari t =  2 ^ \n agar { 5 &MM 5 } \n [ \n e Jam 10^ \n ] \n Begir(\"%d\",  &X )^ \n Benevis(\"Hello World\")^ \n a = (((t Jam 4) Jam  YekiBala k  ) Zarb 7)^ \n ta { 5 &MM 7} [ \n U= 8^ \n ] ",
    // );

    // let contents =
    //     fs::read_to_string("content.txt").expect("Something went wrong reading the file");
    // let mut tokens: Vec<SyntaxToken> = lexer::get_tokens(&contents);

    // println!("{}", contents);

    // for token in &tokens {
    //     println!("text: \"{}\" \n kind: {:?}", token.text, token.kind);
    // }
    let table = parsing_table::parsing_table();
    let next: bool = parser::parse(&mut tokens, table, true);
    if next {
        // Semantic
        semantics::symbol_tab_filler(&mut tokens);
    } else {
        // Show Users Error
    }
}
/*
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
*/
