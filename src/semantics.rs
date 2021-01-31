#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::lexer::SyntaxToken;
use crate::syntax_kinds::SyntaxKind;

#[derive(PartialEq, PartialOrd, Debug)]
struct Symbol {
    types: String,
    name: String,
    value: String,
    scope: String,
}

pub fn symbol_tab_filler(tokens: &mut Vec<SyntaxToken>) {
    let mut flag_condition = false;

    let mut check_1: Vec<&str> = vec![];
    let mut check_2: Vec<&str> = vec![];

    let mut symbol_tab: Vec<Symbol> = vec![];
    let mut _types: &str = "type";
    let mut _name: &str = "name";
    let mut _value: &str = "value";
    let mut _scope: &str = "scope";
    let mut print_sign: &str = "print_sign";
    let mut state: i32 = 0;
    for token in &*tokens {
        match &token.kind {
            SyntaxKind::WhitespaceToken
            | SyntaxKind::QuotationToken
            | SyntaxKind::SingleQouteToken => continue,
            _ => println!("Hi :  {:?} : {}", token.kind, state),
        }
        match state {
            0 => match token.kind {
                SyntaxKind::IntegerDefToken => {
                    _types = "int";
                    state = 1;
                }
                SyntaxKind::FloatDefToken => {
                    _types = "float";
                    state = 1;
                }
                SyntaxKind::CharacterDefToken => {
                    _types = "char";
                    state = 1;
                }
                SyntaxKind::WordlyToken => {
                    // test for existing if ok => state = 2 else state = 0
                    // println!("error: variable {} is not declared", token.text);
                    if check_existing(&token.text, &symbol_tab) {
                        state = 2;
                    } else {
                        state = 0;
                        println!("error: variable {} is not declared", token.text);
                    }
                }
                SyntaxKind::PrintToken => {
                    state = 6;
                }
                SyntaxKind::ScanToken => {
                    state = 13;
                }
                SyntaxKind::ConditionToken | SyntaxKind::LoopToken => {
                    state = 100;
                }
                _ => state = 0,
            },
            1 => match token.kind {
                SyntaxKind::WordlyToken => {
                    // test for existing if ok (if it existed) => state = 0 else state = 2 and name = &*token.text;
                    // println!("error: variable had been declared before");
                    if check_existing(&token.text, &symbol_tab) {
                        println!("error: variable had been declared before");
                        state = 0;
                    } else {
                        _name = &token.text;
                        state = 2;
                    }
                }
                _ => println!("1 , {:?}", token.kind),
            },
            2 => match token.kind {
                SyntaxKind::CaretToken => {
                    // create the struct then push to vector
                    let symbol = Symbol {
                        types: _types.to_string(),
                        name: _name.to_string(),
                        value: _value.to_string(),
                        scope: _scope.to_string(),
                    };
                    symbol_tab.push(symbol);
                    state = 0;
                }
                SyntaxKind::CommaToken => {
                    let symbol = Symbol {
                        types: _types.to_string(),
                        name: _name.to_string(),
                        value: _value.to_string(),
                        scope: _scope.to_string(),
                    };
                    _value = "2123135165464684";
                    symbol_tab.push(symbol);
                    state = 12;
                }
                SyntaxKind::AssignToken => {
                    state = 3;
                }
                SyntaxKind::AdditionToken
                | SyntaxKind::SubstractionToken
                | SyntaxKind::MultiplicationToken
                | SyntaxKind::DivisionToken
                | SyntaxKind::ModulusToken
                | SyntaxKind::IncrementToken
                | SyntaxKind::DecrementToken => {
                    if _types == "char" {
                        println!(
                            "error: can't perform arithmetic operations on 'Harf' On Lin : {}",
                            token.line
                        );
                    } else {
                        state = 5;
                    }
                }
                _ => println!("2 , {:?}", token.kind),
            },
            3 => match token.kind {
                SyntaxKind::NumberToken => {
                    if token.text.contains(".") && _types == "float" {
                        _value = &token.text;
                        state = 4;
                    } else if !token.text.contains(".") && _types == "int" {
                        _value = &token.text;
                        state = 4;
                    } else {
                        println!("error: types mismatched \n variable doesn't have a value matched with the identifier");
                        state = 0;
                    }
                }
                SyntaxKind::CharToken => {
                    if _types == "char" {
                        _value = &token.text;
                        state = 4;
                    } else {
                        println!("error: types mismatched \n variable doesn't have a value matched with the identifier");
                    }
                }
                SyntaxKind::WordlyToken => {
                    // check for existing the var if ok -> value = &*token.text;state = 4;
                    // println!("error: types mismatched \n variable doesn't have a value matched with the identifier");
                    if check_existing(&token.text, &symbol_tab) {
                        _value = &token.text;
                        state = 4;
                    } else {
                        println!("error: variable {} is not declared", token.text);
                        state = 0;
                    }
                }
                _ => println!("3 , {:?}", token.kind),
            },
            4 => match token.kind {
                SyntaxKind::AdditionToken
                | SyntaxKind::SubstractionToken
                | SyntaxKind::MultiplicationToken
                | SyntaxKind::DivisionToken
                | SyntaxKind::ModulusToken
                | SyntaxKind::IncrementToken
                | SyntaxKind::DecrementToken => {
                    if _types == "char" {
                        println!("error: can't perform arithmetic operations on 'Harf'");
                    }
                    state = 0;
                }
                SyntaxKind::CaretToken => {
                    let symbol = Symbol {
                        types: _types.to_string(),
                        name: _name.to_string(),
                        value: _value.to_string(),
                        scope: _scope.to_string(),
                    };
                    symbol_tab.push(symbol);
                    state = 0;
                }
                SyntaxKind::CommaToken => {
                    let symbol = Symbol {
                        types: _types.to_string(),
                        name: _name.to_string(),
                        value: _value.to_string(),
                        scope: _scope.to_string(),
                    };
                    _value = "2123135165464684";
                    symbol_tab.push(symbol);
                    state = 12;
                }
                _ => println!("4 , {:?}", token.kind),
            },
            5 => match token.kind {
                SyntaxKind::WordlyToken => {
                    // ckeck for existing if ok (then check for it type == char if ok -> error) else  err
                    // err => println!("error: variable {} is not declared", token.text); state =0
                    // error => println!("error: can't perform arithmetic operations on 'Harf'"); state =0
                    if check_existing(&token.text, &symbol_tab) {
                        if check_type(&token.text, "char", &symbol_tab) {
                            println!("error: can't perform arithmetic operations on 'Harf' HERE");
                            state = 0;
                        }
                    } else {
                        println!("error: variable {} is not declared", token.text);
                        state = 0;
                    }
                }
                _ => println!("5 , {:?}", token.kind),
            },
            6 => match token.kind {
                SyntaxKind::ParenthesesOpenToken => {
                    state = 8;
                }
                _ => println!("6 , {:?}", token.kind),
            },
            8 => match token.kind {
                SyntaxKind::StringNumToken
                | SyntaxKind::StringCharToken
                | SyntaxKind::StringFloatToken => {
                    check_1.push(&token.text);
                    state = 8;
                }
                SyntaxKind::StringToken => state = 8,
                SyntaxKind::CommaToken => state = 9,
                SyntaxKind::ParenthesesCloseToken => {
                    check(&mut check_1, &mut check_2, &symbol_tab);
                    state = 0;
                }
                _ => println!("8 , {:?}", token.kind),
            },
            9 => match token.kind {
                SyntaxKind::WordlyToken => {
                    check_2.push(&token.text);
                    state = 9
                }
                SyntaxKind::CommaToken => state = 9,
                SyntaxKind::ParenthesesCloseToken => {
                    check(&mut check_1, &mut check_2, &symbol_tab);
                    state = 0;
                }
                _ => println!("9 , {:?}", token.kind),
            },
            12 => match token.kind {
                SyntaxKind::WordlyToken => {
                    _name = &token.text;
                    state = 2;
                }
                _ => println!("12, {:?}", token.kind),
            },
            13 => match token.kind {
                SyntaxKind::ParenthesesOpenToken => {
                    state = 14;
                }
                _ => println!("13 , {:?}", token.kind),
            },
            14 => match token.kind {
                SyntaxKind::StringNumToken
                | SyntaxKind::StringCharToken
                | SyntaxKind::StringFloatToken => {
                    check_1.push(&token.text);
                    state = 15;
                }
                _ => println!("14 , {:?}", token.kind),
            },
            15 => match token.kind {
                SyntaxKind::CommaToken => {
                    state = 16;
                }
                _ => println!("15 , {:?}", token.kind),
            },
            16 => match token.kind {
                SyntaxKind::WordlyToken => {
                    if token.text.contains("&") {
                        state = 18;
                        let name: Vec<&str> = token.text.split("&").collect();
                        for n in name {
                            if n != "" {
                                check_2.push(n);
                            }
                        }
                    } else if token.text == "&".to_string() {
                        state = 17;
                    }
                }
                _ => println!("16 , {:?}", token.kind),
            },
            17 => match token.kind {
                SyntaxKind::WordlyToken => {
                    state = 18;
                }
                _ => println!("OK"),
            },
            18 => match token.kind {
                SyntaxKind::ParenthesesCloseToken => {
                    check(&mut check_1, &mut check_2, &symbol_tab);
                    state = 0;
                }
                _ => println!("Ok"),
            },
            100 => match token.kind {
                SyntaxKind::WordlyToken => {
                    _types = "None";
                    _name = &token.text;
                    for symbol in &symbol_tab {
                        if symbol.name == token.text {
                            if symbol.types == "int" || symbol.types == "float" {
                                _types = "int";
                                state = 101;
                                break;
                            } else if symbol.types == "char" {
                                _types = "char";
                                state = 102;
                                break;
                            }
                        }
                    }
                }
                SyntaxKind::NumberToken => {
                    state = 101;
                }
                SyntaxKind::CharToken => state = 102,
                _ => println!("100 , {:?}", token.kind),
            },
            101 => match token.kind {
                SyntaxKind::LTToken
                | SyntaxKind::LTOEToken
                | SyntaxKind::GTToken
                | SyntaxKind::GTOEToken
                | SyntaxKind::EqualToken => {
                    state = 103;
                    flag_condition = true;
                }
                SyntaxKind::AdditionToken
                | SyntaxKind::MultiplicationToken
                | SyntaxKind::ModulusToken
                | SyntaxKind::SubstractionToken
                | SyntaxKind::DivisionToken => {
                    state = 105;
                }
                SyntaxKind::CloseBracketToken => {
                    println!("Error: Expected Boolean");
                    state = 0;
                }
                _ => println!("101 , {:?}", token.kind),
            },
            102 => match token.kind {
                SyntaxKind::EqualToken => state = 104,
                SyntaxKind::CloseBracketToken => {
                    println!("Error: Expected Boolean");
                    state = 0;
                }
                _ => println!("Error 102"),
            },
            103 => match token.kind {
                SyntaxKind::WordlyToken => {
                    if check_type(&token.text, "int", &symbol_tab)
                        || check_type(&token.text, "float", &symbol_tab)
                    {
                        state = 106;
                    } else {
                        println!("error: 'Number' should be compared with 'Number'");
                        state = 0;
                    }
                }
                SyntaxKind::NumberToken => state = 0,
                _ => println!("Error 103"),
            },
            104 => match token.kind {
                SyntaxKind::WordlyToken => {
                    if check_type(&token.text, "char", &symbol_tab) {
                    } else {
                        println!("error: 'Harf' should be compared with 'Harf'");
                    }
                    state = 0;
                }
                SyntaxKind::CharToken => state = 0,
                _ => println!("Error 104"),
            },
            105 => match token.kind {
                SyntaxKind::WordlyToken => {
                    if check_type(&token.text, "int", &symbol_tab)
                        || check_type(&token.text, "float", &symbol_tab)
                    {
                    } else {
                        println!("Error:Arithmetic 'Harf'");
                    }
                    state = 106;
                }
                SyntaxKind::NumberToken => state = 106,
                SyntaxKind::CharToken => {
                    println!("Error: Arithmetic Comparison 'Harf'");
                    state = 0;
                }
                _ => println!("Error105"),
            },
            106 => match token.kind {
                SyntaxKind::AdditionToken
                | SyntaxKind::MultiplicationToken
                | SyntaxKind::ModulusToken
                | SyntaxKind::SubstractionToken
                | SyntaxKind::DivisionToken => {
                    state = 105;
                }
                SyntaxKind::LTToken
                | SyntaxKind::LTOEToken
                | SyntaxKind::GTToken
                | SyntaxKind::GTOEToken
                | SyntaxKind::EqualToken => {
                    state = 105;
                    flag_condition = true;
                }
                SyntaxKind::CloseBracketToken => {
                    if flag_condition {
                        flag_condition = false;
                    } else {
                        println!("Error:Expected Boolean")
                    }
                    state = 0;
                }
                _ => println!("Error106"),
            },
            _ => println!("Some"),
        }
    }
}

fn check_existing(input: &str, symbol_tab: &Vec<Symbol>) -> bool {
    let mut flag: bool = false;
    println!("{:?}", symbol_tab);
    for symbol in symbol_tab {
        if symbol.name == input.to_string() {
            flag = true;
        }
    }
    flag
}

fn check_type(input: &str, input_type: &str, symbol_tab: &Vec<Symbol>) -> bool {
    let mut flag: bool = false;
    println!("{:?}", symbol_tab);
    for symbol in symbol_tab {
        if symbol.name == input.to_string() {
            if symbol.types == input_type.to_string() {
                flag = true;
            }
        }
    }
    flag
}

fn get_type(input: &str, symbol_tab: &Vec<Symbol>) -> String {
    let mut answer: String = "".to_string();
    for symbol in symbol_tab {
        if symbol.name == input.to_string() {
            if symbol.types == "int" {
                answer = "int".to_string();
            } else if symbol.types == "char" {
                answer = "char".to_string();
            } else if symbol.types == "float" {
                answer = "float".to_string();
            }
        }
    }
    answer
}

fn check(list_1: &mut Vec<&str>, list_2: &mut Vec<&str>, symbol_tab: &Vec<Symbol>) {
    if list_1.len() == list_2.len() {
        loop {
            match list_1.pop() {
                Some(value_1) => match list_2.pop() {
                    Some(value_2) => {
                        if value_1 == "%d" && check_type(value_2, "int", &symbol_tab) {
                        } else if value_1 == "%f" && check_type(value_2, "float", &symbol_tab) {
                        } else if value_1 == "%c" && check_type(value_2, "char", &symbol_tab) {
                        } else {
                            println!("Error:{} and {} does not match", value_1, value_2);
                        }
                    }
                    None => break,
                },
                None => break,
            }
        }
    } else {
        println!("Ridi");
    }
}
