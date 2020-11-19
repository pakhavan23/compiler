/**
 * author: <Erfan Derakhshani>
 * author_email: <techerfan@gmail.com>
 */

pub enum SyntaxKind {
  NumberToken,
  WhitespaceToken,
  QuotationToken,
  CaretToken,
  OpenBracketToken,
  OpenSquareBracketToken,
  CloseBracketToken,
  CloseSquareBracketToken,
}
pub struct SyntaxToken {
  pub line: i32,
  pub position: i32,
  pub text: String,
  pub kind: SyntaxKind
}

pub fn get_tokens(text :&str) -> Vec<SyntaxToken> {
  lexer(text)
}

/**
 * 
 * it gives the whole text and returns a vector of tokens
 * 
 * @param { String } text
 * @returns { Vec<SyntaxToken> }
 */
fn lexer(text: &str) -> Vec<SyntaxToken> {
  let mut tokens: Vec<SyntaxToken> = vec![];
  let lines_vector: Vec<&str> = text.split("\n").collect();

  /* split the code line by line */
  for i in 0..lines_vector.len() {
    let mut position = 0;
    let line = lines_vector[i];
    let end_position = line.len() - 1;
    let chars_vec: Vec<char> = line.chars().collect();
    while position <= end_position {
      if chars_vec[position] == ' ' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::WhitespaceToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == '{' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::OpenBracketToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == '[' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::OpenSquareBracketToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == ']' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::CloseSquareBracketToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == '}' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::CloseBracketToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == '^' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::CaretToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == '"' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::QuotationToken,
          line: i as i32
        };
        tokens.push(token);
      } else {
        let token = word_detector(&chars_vec, &mut position, i as i32);
        tokens.push(token);
      }
      position = position + 1;
    }
  }
  

  /*
   * use this block of code to convert 
   * string to a vector of chars:
   * 
  let name = String::from("erfan");
  let s: Vec<char> = name.chars().collect();
  */
  tokens
}

fn word_detector(chars_vec: &Vec<char>,position: &mut usize, line_number: i32) -> SyntaxToken {
  let mut word: String = String::from("");
  /* we know the current position has a wordly character because we had checked it before.*/
  word.push(chars_vec[*position]);
  for i in *position + 1..chars_vec.len() {
    if chars_vec[i] != ' ' 
      && chars_vec[i] != '{' 
      && chars_vec[i] != '['
      && chars_vec[i] != '}'
      && chars_vec[i] != ']'
      && chars_vec[i] != '^'
      && chars_vec[i] != '"' {
        word.push(chars_vec[i]);
        /* in case that the last character is a wordly char. then we need to change to position to "i". */
        if i == chars_vec.len() - 1 {
          *position = i;  
        }
      } else {
        *position = i - 1;
        break;
      }
  }
  let token = SyntaxToken {
    text: word,
    position: *position as i32,
    kind: SyntaxKind::CaretToken,
    line: line_number
  };
  token
}