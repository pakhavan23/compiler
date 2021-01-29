use crate::lexer::SyntaxToken;
use crate::syntax_kinds::SyntaxKind;
use crate::semantic_errors::semantic_errors;

pub fn symbol_tab_filler(tokens: &mut Vec<SyntaxToken>){
    let mut symbol_tab = vec![["type" , "name" , "value" , "scope"]];
    let mut type , name , value , scope , print_sign;
    let mut state : i32 = 0;
    for token in &*tokens{
        match state {
            0 => match token.kind {
                SyntaxKind::IntegerDefToken => {
                    type = "int";
                    state = 1;
                } , 
                SyntaxKind::FloatDefToken => {
                    type = "float";
                    state = 1;
                } ,
                SyntaxKind::CharacterDefToken => {
                    type = "char";
                    state = 1;
                } ,
                SyntaxKind::WordlyToken => {
                    for symbol in symbol_tab{
                        if(token.text !== symbol[1]){
                            println!("{}" , semantic_errors.undeclaredVariable);
                            state = 0;
                        }
                        else{
                            state = 2;
                        }
                    }
                },
                SyntaxKind::PrintToken => {
                    state = 6;
                }
                _ => state = 0,
            },
            1 => match token.kind {
                SyntaxKind::WordlyToken => {
                    for symbol in symbol_tab{
                        if(token.text == symbol[1]){
                            println!("{}" , semantic_errors.multipleDeclaration);
                            state = 0;
                        }
                    }
                    else{
                        name = token.text;
                        state = 2;
                    }
                },
            }
            2 => match token.kind {
                SyntaxKind::CaretToken => {
                    symbol_tab.push([type , name , " " , " "]);
                    state = 0;
                },
                SyntaxKind::CommaToken => {
                    symbol_tab.push([type , name , " " , " "]);
                    state = 0;
                },
                SyntaxKind::AssignToken => {
                    state = 3;
                },
                SyntaxKind::AdditionToken => {
                    if(type == "char"){
                        println!("{}" , semantic_errors.wrongOperation);
                    }
                    else{
                        state = 5;
                    }
                },
                SyntaxKind::SubstractionToken => {
                    if(type == "char"){
                        println!("{}" , semantic_errors.wrongOperation);
                    }
                    else{
                        state = 5;
                    }
                },
                SyntaxKind::MultiplicationToken => {
                    if(type == "char"){
                        println!("{}" , semantic_errors.wrongOperation);
                    }
                    else{
                        state = 5;
                    }
                },
                SyntaxKind::DivisionToken => {
                    if(type == "char"){
                        println!("{}" , semantic_errors.wrongOperation);
                    }
                    else{
                        state = 5;
                    }
                },
                SyntaxKind::ModulusToken => {
                    if(type == "char"){
                        println!("{}" , semantic_errors.wrongOperation);
                    }
                    else{
                        state = 5;
                    }
                },
                SyntaxKind::IncrementToken => {
                    if(type == "char"){
                        println!("{}" , semantic_errors.wrongOperation);
                    }
                    else{
                        state = 5;
                    }
                },
                SyntaxKind::DecrementToken => {
                    if(type == "char"){
                        println!("{}" , semantic_errors.wrongOperation);
                    }
                    else{
                        state = 5;
                    }
                },
            }
            3 => match token.kind {
                SyntaxKind::NumberToken => {
                    if(token.text.contains(".") && type == "float"){
                        value = token.text;
                        state = 4;
                    }
                    else if(!token.text.contains(".") && type == "int"){
                        value = token.text;
                        state = 4;
                    }
                    else{
                        println!("{}" , semantic_errors.typeMismatched);
                        state = 0;
                    }
                },
                SyntaxKind::CharToken => {
                    if(type == "char"){
                        value = token.text;
                        state = 4;
                    }
                    else{
                        println!("{}" , semantic_errors.typeMismatched);
                    }
                },
                SyntaxKind::WordlyToken => {
                    for symbol in symbol_tab{
                        if(token.text == symbol[1]){
                            value = token.text;
                            state = 4;
                        }
                        else{
                            println!("{}" , semantic_errors.assignMismatched);
                        }
                    }
                }
            }
            4 => match token.kind {
                SyntaxKind::CaretToken => {
                    symbol_tab.push([type , name , value , " "]);
                    state = 0;
                },
                SyntaxKind::CommaToken => {
                    symbol_tab.push([type , name , value , " "]);
                    state = 0;
                },
            }
            5 => match token.kind {
                SyntaxKind::WordlyToken => {
                    for symbol in symbol_tab{
                        if(!token.text == symbol[1]){
                            println!("{}" , semantic_errors.undeclaredVariable);
                            state = 0;
                        }
                        else if(token.text == symbol[1] && symbol[0] == "char"){
                            println!("{}" , semantic_errors.wrongOperation);
                            state = 0;
                        }
                    }
                },
            }
            6 => match token.kind {
                SyntaxKind::ParenthesesOpenToken => {
                    state = 7;
                }
            }
            7 => match token.kind {
                SyntaxKind::QuotationToken => {
                    state = 8;
                }
            }
            8 => match token.kind {
                SyntaxKind::StringNumToken => {
                    print_sign = token.text;
                    state = 9;
                }
            }
            9 => match token.kind {
                SyntaxKind::QuotationToken => {
                    state = 10;
                }
            }
            10 => match token.kind {
                SyntaxKind::CommaToken => {
                    state = 11;
                }
            }
            11 => match token.kind {
                SyntaxKind::WordlyToken => {
                    for symbol in symbol_tab{
                        if(!token.text == symbol[1]){
                            println!("{}" , semantic_errors.undeclaredVariable);
                            state = 0;
                        }
                        else if(token.text == symbol[1] && symbol[0] == "int" && print_sign !== "%d"){
                            println!("{}" , semantic_errors.printMismatched);
                            state = 0;
                        }
                        else if(token.text == symbol[1] && symbol[0] == "float" && print_sign !== "%f"){
                            println!("{}" , semantic_errors.printMismatched);
                            state = 0;
                        }
                        else if(token.text == symbol[1] && symbol[0] == "char" && print_sign !== "%c"){
                            println!("{}" , semantic_errors.printMismatched);
                            state = 0;
                        }
                    }
                }
            }
            
            }
        }
    }

