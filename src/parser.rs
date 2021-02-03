#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::lexer::SyntaxToken;
use crate::syntax_kinds::SyntaxKind;
use std::fs::OpenOptions;

use std::io::prelude::*;

pub fn parse(
    tokens: &mut Vec<SyntaxToken>,
    table: [[&'static str; 33]; 25],
    is_rejected: bool,
) -> bool {
    let mut scanf = false;
    let mut s_state = 0;
    let mut printf = false;
    let mut p_state = 0;

    let mut stack: Vec<&'static str> = vec!["S"];
    let mut line: i32 = 1;
    let mut flag = true;
    let mut token_to_add: SyntaxToken = SyntaxToken {
        text: "^".to_string(),
        position: 1222 as i32,
        kind: SyntaxKind::CaretToken,
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
            SyntaxKind::WhitespaceToken
            | SyntaxKind::QuotationToken
            | SyntaxKind::SingleQouteToken => continue,
            SyntaxKind::WordlyToken => {
                if token.text == "\r" {
                    continue;
                } else {
                    non_terminal_index = get_non_terminal_index(&id, &table);
                }
            }
            SyntaxKind::NumberToken => non_terminal_index = get_non_terminal_index(&num, &table),
            SyntaxKind::StringToken
            | SyntaxKind::StringNumToken
            | SyntaxKind::StringFloatToken
            | SyntaxKind::StringCharToken
            | SyntaxKind::CharToken => non_terminal_index = get_non_terminal_index(&string, &table),
            _ => non_terminal_index = get_non_terminal_index(&text, &table),
        }
        if line < token.line - 1 {
            line += 1;
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
                        println!("{:?}", stack);
                    } else if is_non_terminal(value, &table) {
                        if text == value
                            || (value == "id" && non_terminal_index == 26)
                            || (value == "num" && non_terminal_index == 27)
                            || (value == "str" && non_terminal_index == 28)
                        {
                            break;
                        } else {
                            if (&token.text == ")" && value == ",")
                                || (&token.text == ")" && value == "id")
                            {
                            } else {
                                flag = false;
                                println!(
                                    "error on Line {} And {} , missing {}",
                                    line, &token.text, value
                                );
                                let message = format!("Error on line {} : Missing {}", line, value);
                                log_error(message);
                                let index = tokens.iter().position(|r| r == token).unwrap();
                                index_to_add = index - 1;
                                if value == "{" {
                                    token_to_add = SyntaxToken {
                                        text: "{".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::OpenBracketToken,
                                        line: line as i32,
                                    };
                                } else if value == "=" {
                                    token_to_add = SyntaxToken {
                                        text: "=".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::AssignToken,
                                        line: line as i32,
                                    };
                                } else if value == "(" {
                                    token_to_add = SyntaxToken {
                                        text: "(".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::ParenthesesOpenToken,
                                        line: line as i32,
                                    };
                                } else if value == ")" {
                                    token_to_add = SyntaxToken {
                                        text: text.to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::ParenthesesCloseToken,
                                        line: line as i32,
                                    };
                                } else if value == "[" {
                                    token_to_add = SyntaxToken {
                                        text: "[".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::OpenSquareBracketToken,
                                        line: line as i32,
                                    };
                                } else if value == "]" {
                                    token_to_add = SyntaxToken {
                                        text: "]".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::CloseSquareBracketToken,
                                        line: line as i32,
                                    };
                                } else if value == "}" {
                                    token_to_add = SyntaxToken {
                                        text: "}".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::CloseBracketToken,
                                        line: line as i32,
                                    };
                                } else if value == "^" {
                                    token_to_add = SyntaxToken {
                                        text: "^".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::CaretToken,
                                        line: line as i32,
                                    };
                                } else if value == "str" {
                                    token_to_add = SyntaxToken {
                                        text: "str".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::StringToken,
                                        line: line as i32,
                                    };
                                } else if value == "," {
                                    token_to_add = SyntaxToken {
                                        text: ",".to_string(),
                                        position: position as i32,
                                        kind: SyntaxKind::CommaToken,
                                        line: line as i32,
                                    };
                                }
                                break;
                            }
                        }
                    } else if value == "error" {
                        flag = false;
                        let index = tokens.iter().position(|r| r == token).unwrap();
                        let mut j = index - 1;
                        index_to_add = index - 1;
                        while &tokens[j].text == " " {
                            j -= 1;
                        }
                        if table[0][non_terminal_index] == "num" {
                            println!("{:?}, {} , num", &tokens[j].kind, tokens[j].position);
                            match &tokens[j].kind {
                                SyntaxKind::WordlyToken => {
                                    let mut t = j - 1;
                                    while &tokens[t].text == " " {
                                        t -= 1;
                                    }
                                    match &tokens[t].kind {
                                        SyntaxKind::IntegerDefToken
                                        | SyntaxKind::FloatDefToken
                                        | SyntaxKind::CommaToken => {
                                            println!(
                                                "error Expected = ,between vlaue {} and {}",
                                                text, tokens[j].text
                                            );
                                            let message = format!("Error on line {} : Expected = , between number {} and variable name {}",line,text,tokens[j].text);
                                            log_error(message);
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
                                            let message = format!("Error on line {} : Expected (Jam , Kam , Tagsim , Zarb , Bagimonde , &B , &BM , &K , &KM , &MM) ,between {} and {}", line, text,tokens[j].text);
                                            log_error(message);
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
                                    let message = format!("Error on line {} : Expected (Jam , Kam , Tagsim , Zarb , Bagimonde , &B , &BM , &K , &KM , &MM) ,between number {} and {}", line, text,tokens[j].text);
                                    log_error(message);
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
                                    let message = format!(
                                        "Error on line {} : Expected id and '=' , got {}",
                                        line, text
                                    );
                                    log_error(message);
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
                            match &tokens[j].kind {
                                SyntaxKind::CharacterDefToken
                                | SyntaxKind::IntegerDefToken
                                | SyntaxKind::FloatDefToken => {
                                    println!("Error: Expected variable name after identifier (Ashari,Harf,Sahih)");
                                    let message = format!("Error on line {} :Expected variable name after identifier (Ashari,Harf,Sahih)",line);
                                    log_error(message);
                                    token_to_add = SyntaxToken {
                                        text: "id".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::WordlyToken,
                                    };
                                }
                                SyntaxKind::AssignToken => {
                                    println!("Error: Expected a value after '='");
                                    let message = format!(
                                        "Error on line {} : Expected a value after '='",
                                        line
                                    );
                                    log_error(message);
                                    token_to_add = SyntaxToken {
                                        text: "30".to_string(),
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
                                    let message = format!(
                                        "Error on line {} : Expected a value between {} and {}",
                                        line, tokens[j].text, text
                                    );
                                    log_error(message);
                                    token_to_add = SyntaxToken {
                                        text: "30".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::NumberToken,
                                    };
                                }
                                SyntaxKind::NumberToken | SyntaxKind::WordlyToken => {
                                    token_to_add = SyntaxToken {
                                        text: "^".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::CaretToken,
                                    };
                                }
                                _ => flag = true,
                            }
                            break;
                        } else if table[0][non_terminal_index] == "id" {
                            match &tokens[j].kind {
                                SyntaxKind::WordlyToken => {
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
                                            let message = format!(
                                                "Error on line {} : Expected ',' ,between number {} and {}",
                                                line, text,tokens[j].text
                                            );
                                            log_error(message);
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
                                    let message = format!(
                                        "Error on line {} : Expected (Jam , Kam , Tagsim , Zarb , Bagimonde , &B , &BM , &K , &KM , &MM) , got number {}",
                                        line, text
                                    );
                                    log_error(message);
                                    token_to_add = SyntaxToken {
                                        text: "Jam".to_string(),
                                        position: 0,
                                        line: 0,
                                        kind: SyntaxKind::AdditionToken,
                                    };
                                }
                                _ => flag = true,
                            }
                        } else if table[0][non_terminal_index] == "Begir"
                            || table[0][non_terminal_index] == "Benevis"
                            || table[0][non_terminal_index] == "ta"
                            || table[0][non_terminal_index] == "agar"
                        {
                            println!("Missing ^");
                            let message = format!("Error on line {} : Missing ^", line);
                            log_error(message);
                            token_to_add = SyntaxToken {
                                text: "^".to_string(),
                                position: position as i32,
                                kind: SyntaxKind::CaretToken,
                                line: line as i32,
                            };
                        } else if table[0][non_terminal_index] == "str" {
                            j = j - 1;
                            while &tokens[j].text == " " {
                                j -= 1;
                            }
                            match &tokens[j].kind {
                                SyntaxKind::WordlyToken => {
                                    let mut t = j - 1;
                                    while &tokens[t].text == " " {
                                        t -= 1;
                                    }
                                    match &tokens[t].kind {
                                        SyntaxKind::CharacterDefToken => {
                                            println!(
                                                "error Expected = ,between vlaue {} and {}",
                                                text, tokens[j].text
                                            );
                                            let message = format!("Error on line {} : Expected = , between value {} and variable name {}",line,text,tokens[j].text);
                                            log_error(message);
                                            token_to_add = SyntaxToken {
                                                text: "=".to_string(),
                                                position: 0,
                                                line: 0,
                                                kind: SyntaxKind::AssignToken,
                                            };
                                        }
                                        _ => println!("UnKnown"),
                                    }
                                }
                                _ => println!("Unknown Error :{:?}", &tokens[j].kind),
                            }
                        }
                        break;
                    }
                }
                None => {
                    let index = tokens.iter().position(|r| r == token).unwrap();
                    if index <= tokens.len() - 1 {
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

    if !stack.is_empty() {
        log_error("Missing ^".to_string());
        token_to_add = SyntaxToken {
            text: "^".to_string(),
            position: 1 as i32,
            kind: SyntaxKind::CaretToken,
            line: line as i32,
        };
        tokens.insert(tokens.len() - 1, token_to_add);
        parse(tokens, table, false);
    }

    is_rejected
}

fn is_terminal(input: &'static str, table: &[[&'static str; 33]; 25]) -> bool {
    let mut flag: bool = false;
    for i in 1..25 {
        if table[i][0] == input {
            flag = true;
        }
    }

    flag
}

fn is_non_terminal(input: &'static str, table: &[[&'static str; 33]; 25]) -> bool {
    let mut flag: bool = false;
    for i in 1..33 {
        if table[0][i] == input {
            flag = true;
        }
    }

    flag
}

fn get_terminal_index(input: &'static str, table: &[[&'static str; 33]; 25]) -> usize {
    let mut index: usize = 0;
    for i in 1..25 {
        if table[i][0] == input {
            index = i;
        }
    }
    index
}

fn get_non_terminal_index(input: &String, table: &[[&'static str; 33]; 25]) -> usize {
    let mut index: usize = 0;
    for i in 1..33 {
        if table[0][i] == input {
            index = i;
        }
    }
    index
}

fn log_error(message: String) {
    let args: Vec<String> = std::env::args().collect();
    let p = String::from("errors.txt");
    let mut s = String::from(&args[2]);
    s.push_str(&p);
    let data_path = std::path::Path::new(&s);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&data_path)
        .unwrap();
    writeln!(file, "{}", message);
    // let mut file = match File::open(&data_path) {
    //     Err(e) => panic!("Couldn't open!"),
    //     Ok(file) => file,
    // };

    // match file.write_all(message.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {} ", why),
    //     Ok(_) => println!("Success"),
    // }
    // let mut file_data = String::new();

    // file.read_to_string(&mut file_data);

    // println!("{}", file_data);
}
