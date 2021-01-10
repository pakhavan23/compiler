#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::lexer::SyntaxToken;
use crate::syntax_kinds::SyntaxKind;

pub fn parse(tokens: &mut Vec<SyntaxToken>, table: [[&'static str; 33]; 22], is_rejected: bool) {
    let mut stack: Vec<&'static str> = vec!["S"];
    let mut line: i32 = 1;
    let mut flag = true;
    let mut token_to_add: SyntaxToken = SyntaxToken {
        text: ",".to_string(),
        position: 1222 as i32,
        kind: SyntaxKind::CloseBracketToken,
        line: 54545 as i32,
    };
    let mut index_to_add: usize = 0;

    let id = String::from("id");
    let num = String::from("num");
    let string = String::from("str");
    for token in &*tokens {
        let mut non_terminal_index: usize;
        let mut terminal_index: usize;
        let text = &token.text;
        let kind = &token.kind;
        let position = token.position;
        match token.kind {
            SyntaxKind::WhitespaceToken | SyntaxKind::QuotationToken => continue,
            SyntaxKind::WordlyToken => non_terminal_index = get_non_terminal_index(&id, &table),
            SyntaxKind::NumberToken => non_terminal_index = get_non_terminal_index(&num, &table),
            SyntaxKind::StringToken | SyntaxKind::StringNumToken => {
                non_terminal_index = get_non_terminal_index(&string, &table)
            }
            _ => non_terminal_index = get_non_terminal_index(&text, &table),
        }
        loop {
            match stack.pop() {
                Some(value) => {
                    //println!("value :{} , input: {} ", value, text);
                    if is_terminal(value, &table) {
                        terminal_index = get_terminal_index(value, &table);
                        let production = table[terminal_index][non_terminal_index];
                        let mut pros: Vec<&str> = production.split(" ").collect();
                        for pro in pros.iter().rev() {
                            if pro.to_string() != "" {
                                stack.push(pro);
                            }
                        }
                    //println!("{:?}", stack);
                    } else if is_non_terminal(value, &table) {
                        if text == value
                            || (value == "id" && non_terminal_index == 26)
                            || (value == "num" && non_terminal_index == 27)
                            || (value == "str" && non_terminal_index == 28)
                        {
                            break;
                        } else {
                            println!("error on Line {} And {} , {}", line, value, text);
                        }
                    } else if value == "error" {
                        flag = false;
                        println!("error on Line {} And {} , {} Errrrr", line, value, text);
                        let index = tokens.iter().position(|r| r == token).unwrap();
                        let mut j = index - 1;
                        index_to_add = index - 1;
                        while &tokens[j].text == " " {
                            j -= 1;
                        }
                        if table[0][non_terminal_index] == "num" {
                            println!("{:?}, {}", &tokens[j].kind, tokens[j].position);
                            match &tokens[j].kind {
                                SyntaxKind::WordlyToken => {
                                    println!("error");
                                    let mut t = j - 1;
                                    while &tokens[t].text == " " {
                                        t -= 1;
                                    }
                                    match &tokens[t].kind {
                                        SyntaxKind::CharacterDefToken
                                        | SyntaxKind::IntegerDefToken
                                        | SyntaxKind::FloatDefToken
                                        | SyntaxKind::CommaToken => {
                                            println!(
                                                "error Expected = ,between number {} and {}",
                                                text, tokens[j].text
                                            );
                                            token_to_add = SyntaxToken {
                                                text: "=".to_string(),
                                                position: 0,
                                                line: 0,
                                                kind: SyntaxKind::AssignToken,
                                            };
                                        }
                                        SyntaxKind::AssignToken
                                        | SyntaxKind::OpenBracketToken
                                        | SyntaxKind::OpenSquareBracketToken
                                        | SyntaxKind::ParenthesesOpenToken => {
                                            println!("error Expected (Jam , Kam , Tagsim , Zarb , Bagimonde , &B , &BM , &K , &KM , &MM) ,between number {} and {}", text,tokens[j].text);
                                            token_to_add = SyntaxToken {
                                                text: "Jam".to_string(),
                                                position: 0,
                                                line: 0,
                                                kind: SyntaxKind::AdditionToken,
                                            };
                                        }
                                        _ => println!("!"),
                                    }
                                }
                                SyntaxKind::NumberToken => {
                                    println!("error Expected (Jam , Kam , Tagsim , Zarb , Bagimonde , &B , &BM , &K , &KM , &MM) , got number {}", text);
                                    token_to_add = SyntaxToken {
                                        text: "Jam".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::AdditionToken,
                                    };
                                }
                                SyntaxKind::CharacterDefToken
                                | SyntaxKind::IntegerDefToken
                                | SyntaxKind::FloatDefToken => {
                                    println!("error Expected id and '=' , got {}", text);
                                    token_to_add = SyntaxToken {
                                        text: "id".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::WordlyToken,
                                    };
                                }
                                _ => flag = true,
                            }
                        } else if table[0][non_terminal_index] == "^" {
                            println!("{:?}, {}", &tokens[j].kind, tokens[j].position);
                            match &tokens[j].kind {
                                SyntaxKind::CharacterDefToken
                                | SyntaxKind::IntegerDefToken
                                | SyntaxKind::FloatDefToken => {
                                    println!("Error: Expected variable name after identifier (Ashari,Harf,Sahih)");
                                    token_to_add = SyntaxToken {
                                        text: "id".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::WordlyToken,
                                    };
                                }
                                SyntaxKind::AssignToken => {
                                    println!("Error: Expected a value after '='");
                                    token_to_add = SyntaxToken {
                                        text: "value".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::NumberToken,
                                    };
                                }
                                SyntaxKind::AdditionToken
                                | SyntaxKind::SubstractionToken
                                | SyntaxKind::MultiplicationToken
                                | SyntaxKind::DivisionToken => {
                                    println!(
                                        "Error: Expected a value between {} and {}",
                                        tokens[j].text, text
                                    );
                                    token_to_add = SyntaxToken {
                                        text: "value".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::NumberToken,
                                    };
                                }
                                _ => println!("!"),
                            }
                        } else if table[0][non_terminal_index] == "id" {
                            println!("{:?}, {}", &tokens[j].kind, tokens[j].position);
                            match &tokens[j].kind {
                                SyntaxKind::WordlyToken => {
                                    println!("error10");
                                    let mut t = j - 1;
                                    while &tokens[t].text == " " {
                                        t -= 1;
                                    }
                                    match &tokens[t].kind {
                                        SyntaxKind::CharacterDefToken
                                        | SyntaxKind::IntegerDefToken
                                        | SyntaxKind::FloatDefToken
                                        | SyntaxKind::CommaToken
                                        | SyntaxKind::WordlyToken => {
                                            println!(
                                                "error Expected ',' ,between number {} and {}",
                                                text, tokens[j].text
                                            );
                                            token_to_add = SyntaxToken {
                                                text: ",".to_string(),
                                                position: 0,
                                                line: 0,
                                                kind: SyntaxKind::CommaToken,
                                            };
                                        }
                                        _ => println!("!"),
                                    }
                                }
                                SyntaxKind::NumberToken => {
                                    println!("error Expected (Jam , Kam , Tagsim , Zarb , Bagimonde , &B , &BM , &K , &KM , &MM) , got number {}", text);
                                    token_to_add = SyntaxToken {
                                        text: "Jam".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::AdditionToken,
                                    };
                                }
                                _ => flag = true,
                            }
                        }
                        break;
                    }
                }
                None => {
                    let index = tokens.iter().position(|r| r.text == token.text).unwrap();
                    if index != tokens.len() - 1 {
                        stack.push("S")
                    }
                }
            }
            if !flag {
                break;
            }
        }
        if !flag {
            break;
        }
    }
    if !flag {
        tokens.insert(index_to_add, token_to_add);
        parse(tokens, table, false);
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
