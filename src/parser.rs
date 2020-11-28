use crate::lexer::SyntaxKind;
use crate::lexer::SyntaxToken;

pub fn parse(tokens: Vec<SyntaxToken>){                      //Function that parses the received tokens
    let mut state: i32 = 0;
    for token in tokens{
        println!("{}" , state);
        match state {                                           //matching the token with defined language syntax
            0 => match token.kind{                              
                SyntaxKind::WhitespaceToken => state = 0,       //first token is space
                SyntaxKind::IntegerDefToken => state = 1,       //first token is Sahih
                SyntaxKind::FloatDefToken => state = 5,         //first token is Ashari
                SyntaxKind::CharacterDefToken => state = 9,     //first token is Harf
                SyntaxKind::PrintToken => state = 13,           //first token is Benevis
                _ => println!("No state0"),
            }
            1 => match token.kind{                              //checks syntax for declaring variables
                SyntaxKind::WhitespaceToken => state = 1,
                SyntaxKind::WordlyToken => state = 2,
                _ => println!("No state1"),
            }
            2 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 2,
                SyntaxKind::CaretToken => state = 0,
                SyntaxKind::CommaToken => state = 1,
                SyntaxKind::AssignToken => state = 3,
                _ => println!("No state2"),
            }
            3 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 3,
                SyntaxKind::NumberToken => state = 4,
                _ => println!("No state3"),
            }
            4 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 4,
                SyntaxKind::CommaToken => state = 1,
                SyntaxKind::CaretToken => state = 0,
                _ => println!("No state4"),
            }
            5 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 5,
                SyntaxKind::WordlyToken => state = 6,
                _ => println!("No state5"),
            }
            6 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 6,
                SyntaxKind::CaretToken => state = 0,
                SyntaxKind::CommaToken => state = 5,
                SyntaxKind::AssignToken => state = 7,
                _ => println!("No state6"),
            }
            7 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 7,
                SyntaxKind::NumberToken => state = 8,
                _ => println!("No state7"),
            }
            8 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 8,
                SyntaxKind::CommaToken => state = 5,
                SyntaxKind::CaretToken => state = 0,
                _ => println!("No state8"),
            }
            9 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 9,
                SyntaxKind::WordlyToken => state = 10,
                _ => println!("No state9"),
            }
            10 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 10,
                SyntaxKind::CaretToken => state = 0,
                SyntaxKind::CommaToken => state = 9,
                SyntaxKind::AssignToken => state = 11,
                _ => println!("No state10"),
            }
            11 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 11,
                SyntaxKind::StringToken => state = 12,
                _ => println!("No state11"),
            }
            12 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 12,
                SyntaxKind::CommaToken => state = 9,
                SyntaxKind::CaretToken => state = 0,
                _ => println!("No state12"),
            }
            13 => match token.kind{                                //checks syntax for Benevis()            
                SyntaxKind::WhitespaceToken => state = 13,
                SyntaxKind::ParenthesesOpenToken => state = 14,
                _ => println!("No state13"),
            }
            14 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 14,
                SyntaxKind::StringToken => state = 15,
                SyntaxKind::WordlyToken => state = 15,
                _ => println!("No state14"),
            }
            15 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 15,
                SyntaxKind::ParenthesesCloseToken => state = 16,
                _ => println!("No state15"),
            }
            16 => match token.kind{
                SyntaxKind::WhitespaceToken => state = 16,
                SyntaxKind::CaretToken => state = 0,
                _ => println!("No state16"),
            }
            _ => println!("Nothing!"),
        }
    }
}