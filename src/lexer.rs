/**
 * author: <Erfan Derakhshani>
 * author_email: <techerfan@gmail.com>
 */

use crate::syntax_kinds::SyntaxKind;

pub struct SyntaxToken {
  pub line: i32,
  pub position: i32,
  pub text: String,
  pub kind: SyntaxKind
}

struct SyntaxDefiner {
  text: String,
  kind: SyntaxKind 
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
    let mut end_position = 0;
    if line.len() > 0 {
      end_position = line.len() - 1;
    }
    let chars_vec: Vec<char> = line.chars().collect();
    while position <= end_position && chars_vec.len() > 0{
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
      } else if chars_vec[position] == '=' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::AssignToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == '(' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::ParenthesesOpenToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == ')' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::ParenthesesCloseToken,
          line: i as i32
        };
        tokens.push(token);
      } else if chars_vec[position] == ',' {
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::CommaToken,
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
      } else if chars_vec[position] == '\''{
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::SingleQouteToken,
          line: i as i32
        };
        tokens.push(token);
        /* the existense of a character depends on the existense of the second ' mark.*/
        if chars_vec[position + 2] == '\'' {
          tokens.push(SyntaxToken {
            text: chars_vec[position + 1].to_string(),
            position: (position + 1) as i32,
            kind: SyntaxKind::CharToken,
            line: i as i32
          });
          tokens.push(SyntaxToken {
            text: chars_vec[position + 2].to_string(),
            position: (position + 2) as i32,
            kind: SyntaxKind::SingleQouteToken,
            line: i as i32
          });
          position = position + 2;
        }
      } else if chars_vec[position] == '"' {
        let mut string_word = String::new();
        let mut flag = false;
        let mut new_position = 0;
        for j in position + 1..chars_vec.len() {
          if chars_vec[j] == '"' {
            string_word = chars_vec[position + 1..j].iter().collect();
            new_position = j;
            flag = true;
            break;
          }
        }
        let token = SyntaxToken {
          text: chars_vec[position].to_string(),
          position: position as i32,
          kind: SyntaxKind::QuotationToken,
          line: i as i32
        };
        tokens.push(token);
        if flag { 
          let string_tokens = get_string_tokens(string_word.clone(), position + 1, i);
          
          for token in string_tokens {
            tokens.push(token);
          }

          tokens.push(SyntaxToken {
            text: chars_vec[new_position].to_string(),
            position: new_position as i32,
            kind: SyntaxKind::QuotationToken,
            line: i as i32
          });

          position = new_position
        }

      } else {
        let word_tokens = word_detector(&chars_vec, &mut position, i as i32);
        for token in word_tokens {
          tokens.push(token);
        }
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

fn word_detector(chars_vec: &Vec<char>,position: &mut usize, line_number: i32) -> Vec<SyntaxToken> {
  let mut tokens: Vec<SyntaxToken> = vec![];
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
      && chars_vec[i] != '('
      && chars_vec[i] != ')'
      && chars_vec[i] != ','
      && chars_vec[i] != '='
      && chars_vec[i] != '\''
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
  if is_number(word.chars().collect()) {
    tokens.push(SyntaxToken {
      text: word,
      position: *position as i32,
      kind: SyntaxKind::NumberToken,
      line: line_number
    })
  } else {
    let wordly_tokens = get_syntax(word.clone(), *position, line_number as usize);
    for token in wordly_tokens {
      tokens.push(token);
    }
  }
  tokens
}

fn is_number(chars_vec: Vec<char>) -> bool {
  let digits_arr = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
  let mut is_num = true;
  let mut dots_counter = 0;
  for i in 0..chars_vec.len() {
    let mut is_digit = false;
    for j in 0..digits_arr.len() {
      if chars_vec[i] == digits_arr[j] {
        is_digit = true;
        break;
      }
    }
    if i == 0 || i == chars_vec.len() - 1 {
      if !is_digit {
        is_num = false;
      }
    } else {
      if !is_digit && chars_vec[i] != '.' {
        is_num = false;
      } else if !is_digit && chars_vec[i] == '.' {
        dots_counter = dots_counter + 1;
        if dots_counter > 1 {
          is_num = false;
        }
      }
    }
  }
  is_num
}

fn get_syntax(phrase: String, position: usize, line: usize) -> Vec<SyntaxToken> {
  let syntax_arr = [ 
    SyntaxDefiner{
      text: "&BM".to_string(),
      kind: SyntaxKind::GTOEToken
    },
    SyntaxDefiner{
      text: "&B".to_string(),
      kind: SyntaxKind::GTToken
    },
    SyntaxDefiner{
      text: "&KM".to_string(),
      kind: SyntaxKind::LTOEToken
    },
    SyntaxDefiner{
      text: "&K".to_string(),
      kind: SyntaxKind::LTToken
    },
    SyntaxDefiner{
      text: "&MM".to_string(),
      kind: SyntaxKind::EqualToken
    },
    SyntaxDefiner{
      text: "Jam".to_string(),
      kind: SyntaxKind::AdditionToken
    },
    SyntaxDefiner{
      text: "YekiBala".to_string(),
      kind: SyntaxKind::IncrementToken
    },
    SyntaxDefiner{
      text: "Kam".to_string(),
      kind: SyntaxKind::SubstractionToken
    },
    SyntaxDefiner{
      text: "YekiPain".to_string(),
      kind: SyntaxKind::DecrementToken
    },
    SyntaxDefiner{
      text: "Zarb".to_string(),
      kind: SyntaxKind::MultiplicationToken
    },
    SyntaxDefiner{
      text: "Tagsim".to_string(),
      kind: SyntaxKind::DivisionToken
    },
    SyntaxDefiner{
      text: "Bagimonde".to_string(),
      kind: SyntaxKind::ModulusToken
    },
    SyntaxDefiner{
      text: "Benevis".to_string(),
      kind: SyntaxKind::PrintToken
    },
    SyntaxDefiner{
      text: "Begir".to_string(),
      kind: SyntaxKind::ScanToken
    },
    SyntaxDefiner{
      text: "agar".to_string(),
      kind: SyntaxKind::ConditionToken
    },
    SyntaxDefiner{
      text: "ta".to_string(),
      kind: SyntaxKind::LoopToken
    },
    SyntaxDefiner{
      text: "Sahih".to_string(),
      kind: SyntaxKind::IntegerDefToken
    },
    SyntaxDefiner{
      text: "Ashari".to_string(),
      kind: SyntaxKind::FloatDefToken
    },
    SyntaxDefiner{
      text: "Harf".to_string(),
      kind: SyntaxKind::CharacterDefToken
    },
  ];
  let mut tokens: Vec<SyntaxToken> =vec![];
  let mut is_syntax = false; 

  for syntax in syntax_arr.iter() {
    if phrase.contains(&syntax.text) {
      is_syntax = true;
      let index = phrase.find(&syntax.text).unwrap();
      if index > 0 {
        if is_number(phrase[0..index].to_string().chars().collect()) {
          tokens.push(SyntaxToken {
            line: line as i32,
            position: position as i32,
            text: phrase[0..index].to_string().clone(),
            kind: SyntaxKind::NumberToken
          });
        } else {
          let v = get_syntax(phrase[0..index].to_string(), position, line);
          if v.len() > 0 {
            for item in v {
              tokens.push(item);
            }
          } else {
            tokens.push(SyntaxToken {
              line: line as i32,
              position: position as i32,
              text: phrase[0..index].to_string().clone(),
              kind: SyntaxKind::WordlyToken
            });
          }
        }
      }

      tokens.push(SyntaxToken {
        line: line as i32,
        position: (position + index) as i32,
        text: syntax.text.clone(),
        kind: syntax.kind.copy()
      });

      if index + syntax.text.len() < phrase.len() - 1 {
        if is_number(phrase[index + syntax.text.len()..phrase.len()].to_string().chars().collect()) {
          tokens.push(SyntaxToken {
            line: line as i32,
            position: (position + index + syntax.text.len()) as i32,
            text: phrase[index + syntax.text.len()..phrase.len()].to_string().clone(),
            kind: SyntaxKind::NumberToken
          });
        } else {
          let v = get_syntax(phrase[index + syntax.text.len()..phrase.len()].to_string(), position + index + syntax.text.len(), line);
          if v.len() > 0 {
            for item in v {
              tokens.push(item);
            }
          } else {
            tokens.push(SyntaxToken {
              line: line as i32,
              position: (position + index + syntax.text.len()) as i32,
              text: phrase[index + syntax.text.len()..phrase.len()].to_string().clone(),
              kind: SyntaxKind::WordlyToken
            });
          }
        }
      }
      break;
    }
  }
  if !is_syntax {
    tokens.push(SyntaxToken {
      line: line as i32,
      position: position as i32,
      text: phrase,
      kind: SyntaxKind::WordlyToken
    });
  }
  tokens
}

fn get_string_tokens(phrase: String, position: usize, line: usize) -> Vec<SyntaxToken> {
  let mut tokens: Vec<SyntaxToken> = vec![];
  let mut is_syntax = false;
  let syntax_arr = [
    SyntaxDefiner {
      text: "%d".to_string(),
      kind: SyntaxKind::StringNumToken
    }
  ];

  for syntax in syntax_arr.iter() {
    if phrase.contains(&syntax.text) {
      is_syntax = true;
      let index = phrase.find(&syntax.text).unwrap();
      if index > 0 {
        let v = get_string_tokens(phrase[0..index].to_string(), position, line);
        if v.len() > 0 {
          for item in v {
            tokens.push(item);
          }
        } else {
          tokens.push(SyntaxToken {
            line: line as i32,
            position: position as i32,
            text: phrase[0..index].to_string().clone(),
            kind: SyntaxKind::StringToken
          });
        }
      }

      tokens.push(SyntaxToken {
        line: line as i32,
        position: (position + index) as i32,
        text: syntax.text.clone(),
        kind: syntax.kind.copy()
      });

      if index + syntax.text.len() < phrase.len() - 1 {
        let v = get_string_tokens(phrase[index + syntax.text.len()..phrase.len()].to_string(), position + index + syntax.text.len(), line);
        if v.len() > 0 {
          for item in v {
            tokens.push(item);
          }
        } else {
          tokens.push(SyntaxToken {
            line: line as i32,
            position: (position + index + syntax.text.len()) as i32,
            text: phrase[index + syntax.text.len()..phrase.len()].to_string().clone(),
            kind: SyntaxKind::StringToken
          });
        }
      }
      break;
    }
  }
  if !is_syntax {
    tokens.push(SyntaxToken {
      line: line as i32,
      position: position as i32,
      text: phrase,
      kind: SyntaxKind::StringToken
    });
  }
  tokens
}
