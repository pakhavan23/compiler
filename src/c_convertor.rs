use crate::lexer::SyntaxToken;
use crate::syntax_kinds::SyntaxKind;

pub fn convert_to_c(tokens:  Vec<SyntaxToken>) -> String {
  let mut line: i32 = 0;
  let mut content = String::from("#include <stdio.h>\n#include <conio.h>\n#include <stdlib.h>\n\nint main()\n{\n  ");
  for token in tokens {
    //println!("position: {} line: {} text: {}", token.position, token.line, token.text);
    if token.line > line {
      line = token.line;
      content.push('\n');
      content.push_str("  ");
    }
    match token.kind {
      SyntaxKind::CaretToken => content.push_str(";"),                               
      SyntaxKind::OpenBracketToken => content.push_str("("),        
      SyntaxKind::OpenSquareBracketToken => content.push_str("{"),  
      SyntaxKind::CloseBracketToken => content.push_str(")"),       
      SyntaxKind::CloseSquareBracketToken => content.push_str("}"), 
      SyntaxKind::AdditionToken => content.push_str("+"),
      SyntaxKind::MultiplicationToken => content.push_str("*"),
      SyntaxKind::SubstractionToken => content.push_str("-"),       
      SyntaxKind::IncrementToken => content.push_str("++"),       
      SyntaxKind::DecrementToken => content.push_str("--"),           
      SyntaxKind::DivisionToken => content.push_str("/"),           
      SyntaxKind::ModulusToken => content.push_str("%"),            
      SyntaxKind::AssignToken => content.push_str("="),             
      SyntaxKind::GTToken => content.push_str(">"),
      SyntaxKind::LTToken => content.push_str("<"),                 
      SyntaxKind::GTOEToken => content.push_str(">="),                
      SyntaxKind::LTOEToken => content.push_str("<="),                
      SyntaxKind::EqualToken => content.push_str("=="),               
      SyntaxKind::PrintToken => content.push_str("printf"),                              
      SyntaxKind::ScanToken => content.push_str("scanf"),                            
      SyntaxKind::ConditionToken => content.push_str("if"),                   
      SyntaxKind::LoopToken => content.push_str("while"),                    
      SyntaxKind::FloatDefToken => content.push_str("float"),                         
      SyntaxKind::IntegerDefToken => content.push_str("int"),                    
      SyntaxKind::CharacterDefToken => content.push_str("char"),   
      _ => content.push_str(&*token.text)
    }
  } 
  content.push_str("\n  printf(\"\\npress any key...\");\n  printf(\"%c\", getch());\n  return 0;\n}");
  return content;
}