#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::lexer::SyntaxToken;
use crate::syntax_kinds::SyntaxKind;

pub fn parse(tokens: Vec<SyntaxToken>, table: [[&'static str; 33]; 22]) {
    let mut stack: Vec<&'static str> = vec!["$", "S"];
    let id = String::from("id");
    let num = String::from("num");
    let string = String::from("str");
    let mut line: i32 = 1;
    for token in tokens {
        let mut non_terminal_index: usize;
        let text = token.text;
        match token.kind {
            SyntaxKind::WhitespaceToken | SyntaxKind::QuotationToken => continue,
            SyntaxKind::WordlyToken => non_terminal_index = get_non_terminal_index(&id, &table),
            SyntaxKind::NumberToken => non_terminal_index = get_non_terminal_index(&num, &table),
            SyntaxKind::StringToken | SyntaxKind::StringNumToken => {
                non_terminal_index = get_non_terminal_index(&string, &table)
            }
            _ => non_terminal_index = get_non_terminal_index(&text, &table),
        }
        let kind = token.kind;
        let position = token.position;
        if line < token.line + 1 {
            line += 1;
            stack.push("S");
        }
        loop {
            match stack.pop() {
                Some(value) => {
                    //println!("value :{} , input: {} ", value, text);
                    if is_terminal(value, &table) {
                        let terminal_index: usize = get_terminal_index(value, &table);
                        let production = table[terminal_index][non_terminal_index];
                        let mut pros: Vec<&str> = production.split(" ").collect();
                        for pro in pros.iter().rev() {
                            if pro.to_string() != "" {
                                stack.push(pro);
                            }
                        }
                    //println!("{:?}", stack);
                    } else if is_non_terminal(value, &table) {
                        if text == value.to_string()
                            || (value == "id" && non_terminal_index == 26)
                            || (value == "num" && non_terminal_index == 27)
                            || (value == "str" && non_terminal_index == 28)
                        {
                            break;
                        } else {
                            println!("error on Line {} And {}", line, value);
                        }
                    } else if value == "error" {
                        if table[0][non_terminal_index] != "$" {
                            println!(
                                "error on Line {} And {} And Position {}",
                                line, table[0][non_terminal_index], position
                            );
                        }
                    }
                }
                None => break,
            }
        }
    }
}

fn is_terminal(input: &'static str, table: &[[&'static str; 33]; 22]) -> bool {
    let mut flag: bool = false;
    for i in 1..22 {
        if table[i][0] == input {
            flag = true;
        }
    }

    flag
}

fn is_non_terminal(input: &'static str, table: &[[&'static str; 33]; 22]) -> bool {
    let mut flag: bool = false;
    for i in 1..33 {
        if table[0][i] == input {
            flag = true;
        }
    }

    flag
}

fn get_terminal_index(input: &'static str, table: &[[&'static str; 33]; 22]) -> usize {
    let mut index: usize = 0;
    for i in 1..22 {
        if table[i][0] == input {
            index = i;
        }
    }
    index
}

fn get_non_terminal_index(input: &String, table: &[[&'static str; 33]; 22]) -> usize {
    let mut index: usize = 0;
    for i in 1..33 {
        if table[0][i] == input {
            index = i;
        }
    }
    index
}
