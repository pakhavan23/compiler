pub enum SyntaxKind {
  UnknownToken,
  WordlyToken,
  NumberToken,                              //  Number like: 12 or 1.2
  StringToken,                              //  String like "Sina#"
  CharToken,                                //  A character
  WhitespaceToken,                          //   :D
  QuotationToken,                           //  "
  SingleQouteToken,                         //  '
  CaretToken,                               //  ^
  OpenBracketToken,                         //  }
  OpenSquareBracketToken,                   //  [
  CloseBracketToken,                        //  }
  CloseSquareBracketToken,                  //  ]
  AdditionToken,                            //  +
  SubstractionToken,                        //  -
  IncrementToken,                           //  ++
  DecrementToken,                           //  --
  MultiplicationToken,                      //  *
  DivisionToken,                            //  /
  ModulusToken,                             //  %
  AssignToken,                              //  =
  GTToken,                                  //  >
  LTToken,                                  //  <
  GTOEToken,                                //  >=
  LTOEToken,                                //  <=
  EqualToken,                               //  ==
  ParenthesesOpenToken,                     //  (
  ParenthesesCloseToken,                    //  )
  CommaToken,                               //  ,
  StringNumToken,                           //  %d
  StringCharToken,                          //  %c
  StringFloatToken,                         //  %f
  PrintToken,                               //  Benevis -> printf
  ScanToken,                                //  Begir -> scanf
  ConditionToken,                           //  agar -> if
  LoopToken,                                //  ta -> while
  FloatDefToken,                            //  Ashari -> float
  IntegerDefToken,                          //  Sahih -> int
  CharacterDefToken,                        //  Harf -> char
}

impl SyntaxKind {
  pub fn copy(&self) -> SyntaxKind {
    match self {
      SyntaxKind::StringNumToken => SyntaxKind::StringNumToken,
      SyntaxKind::StringCharToken => SyntaxKind::StringCharToken,
      SyntaxKind::StringFloatToken => SyntaxKind::StringFloatToken,
      SyntaxKind::SingleQouteToken => SyntaxKind::SingleQouteToken,
      SyntaxKind::CharToken => SyntaxKind::CharToken,
      SyntaxKind::ParenthesesCloseToken => SyntaxKind::ParenthesesCloseToken,
      SyntaxKind::ParenthesesOpenToken => SyntaxKind::ParenthesesOpenToken,
      SyntaxKind::CommaToken => SyntaxKind::CommaToken,
      SyntaxKind::WordlyToken => SyntaxKind::WordlyToken,
      SyntaxKind::NumberToken => SyntaxKind::NumberToken,
      SyntaxKind::StringToken => SyntaxKind::StringToken,                              
      SyntaxKind::WhitespaceToken => SyntaxKind::WhitespaceToken,           
      SyntaxKind::QuotationToken => SyntaxKind::QuotationToken,          
      SyntaxKind::CaretToken => SyntaxKind::CaretToken,                               
      SyntaxKind::OpenBracketToken => SyntaxKind::OpenBracketToken,        
      SyntaxKind::OpenSquareBracketToken => SyntaxKind::OpenSquareBracketToken,  
      SyntaxKind::CloseBracketToken => SyntaxKind::CloseBracketToken,       
      SyntaxKind::CloseSquareBracketToken => SyntaxKind::CloseSquareBracketToken, 
      SyntaxKind::AdditionToken => SyntaxKind::AdditionToken,           
      SyntaxKind::SubstractionToken => SyntaxKind::SubstractionToken,       
      SyntaxKind::IncrementToken => SyntaxKind::IncrementToken,           
      SyntaxKind::DecrementToken => SyntaxKind::DecrementToken,           
      SyntaxKind::MultiplicationToken => SyntaxKind::MultiplicationToken,     
      SyntaxKind::DivisionToken => SyntaxKind::DivisionToken,           
      SyntaxKind::ModulusToken => SyntaxKind::ModulusToken,            
      SyntaxKind::AssignToken => SyntaxKind::AssignToken,             
      SyntaxKind::GTToken => SyntaxKind::GTToken,
      SyntaxKind::LTToken => SyntaxKind::LTToken,                 
      SyntaxKind::GTOEToken => SyntaxKind::GTOEToken,                
      SyntaxKind::LTOEToken => SyntaxKind::LTOEToken,                
      SyntaxKind::EqualToken => SyntaxKind::EqualToken,               
      SyntaxKind::PrintToken => SyntaxKind::PrintToken,                              
      SyntaxKind::ScanToken => SyntaxKind::ScanToken,                            
      SyntaxKind::ConditionToken => SyntaxKind::ConditionToken,                   
      SyntaxKind::LoopToken => SyntaxKind::LoopToken,                         
      SyntaxKind::FloatDefToken => SyntaxKind::FloatDefToken,                         
      SyntaxKind::IntegerDefToken => SyntaxKind::IntegerDefToken,                    
      SyntaxKind::CharacterDefToken => SyntaxKind::CharacterDefToken,
      _ => SyntaxKind::UnknownToken,     
    }
  }
}