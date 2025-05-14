//Created by: Nisha Murali (Requested not to clone / re-use the code)
// error.rs (Handles all the parsing errors)
#[derive(Debug)]
pub enum ParseError 
{
    UnexpectedEnd,
    ExpectedKeyword(String),
    ExpectedIdentifier,
    InvalidExpression(String),
    UnknownStartOfStatement(String),
    General(String),
}

impl std::fmt::Display for ParseError 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            ParseError::UnexpectedEnd => write!(f, "Unexpected end of input, Please check once again"),
            ParseError::ExpectedKeyword(k) => write!(f, "Expected keyword: {}", k),
            ParseError::ExpectedIdentifier => write!(f, "Expected an identifier, Check once again"),
            ParseError::InvalidExpression(e) => write!(f, "Invalid expression: {}", e),
            ParseError::UnknownStartOfStatement(t) => write!(f, "Unknown start of statement: {}", t),
            ParseError::General(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ParseError {}
