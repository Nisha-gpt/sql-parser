//Created by: Nisha Murali (Requested not to clone / re-use the code)
//parser.rs ( Parses SELECT and CREATE TABLE into abstarct syntax tree )
use crate::tokenizer::{Token, Keyword};
use crate::ast::{Statement, Expression, DBType, TableColumn, Constraint};
use crate::pratt::PrattParser;
use crate::error::ParseError;

pub struct SQLParser<'a> 
{
    tokens: &'a [Token],
    position: usize,
}

impl<'a> SQLParser<'a> 
{
    pub fn new(tokens: &'a [Token]) -> Self 
    {
        Self { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> 
    {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> 
    {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> Result<(), ParseError> 
    {
        match self.advance() 
        {
            Some(Token::Keyword(k)) if *k == keyword => Ok(()),
            Some(_) => Err(ParseError::ExpectedKeyword(format!("{:?}", keyword))),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> 
    {
        match self.advance() 
        {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            Some(_) => Err(ParseError::ExpectedIdentifier),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> 
    {
        match self.advance() 
        {
            Some(t) if *t == expected => Ok(()),
            Some(t) => Err(ParseError::General(format!("Expected {:?}, got {:?}", expected, t))),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> 
    {
        match self.peek() 
        {
            Some(Token::Keyword(Keyword::Select)) => self.parse_select(),
            Some(Token::Keyword(Keyword::Create)) => self.parse_create_table(),
            Some(tok) => Err(ParseError::UnknownStartOfStatement(format!("{:?}", tok))),
            None => Err(ParseError::General("Empty input".to_string())),
        }
    }

    fn parse_select(&mut self) -> Result<Statement, ParseError> 
    {
        self.expect_keyword(Keyword::Select)?;

        let mut columns = Vec::new();
        loop 
        {
            match self.advance() 
            {
                Some(Token::Identifier(name)) => columns.push(name.clone()),
                Some(Token::Comma) => continue,
                Some(Token::Keyword(Keyword::From)) => break,
                Some(tok) => 
                {
                    return Err(ParseError::General(format!(
                        "Unexpected token in column list: {:?}", tok)))
                }
                None => return Err(ParseError::UnexpectedEnd),
            }
        }

        let table = self.expect_identifier()?;

        let mut selection = None;
        if let Some(Token::Identifier(name)) = self.peek() {
    if name == "*" {
        self.advance();
        columns.push("*".to_string());
        // Skip any trailing commas
        while let Some(Token::Comma) = self.peek() {
            self.advance();
        }
        self.expect_keyword(Keyword::From)?;
    }
}

        //if let Some(Token::Keyword(Keyword::Where)) = self.peek() 
        //{
            //self.advance();
            //let remaining = &self.tokens[self.position..];
            //let mut expr_parser = PrattParser::new(remaining);
           // let expr = expr_parser.parse_expression(1)
             //   .map_err(ParseError::InvalidExpression)?;
            //self.position += expr_parser.position;
            //selection = Some(expr);
        //}

        let mut order_by = None;
        if let Some(Token::Keyword(Keyword::Order)) = self.peek() 
        {
            self.advance();
            self.expect_keyword(Keyword::By)?;
            let mut cols = Vec::new();
            loop 
            {
                match self.advance() 
                {
                    Some(Token::Identifier(name)) => cols.push(name.clone()),
                    Some(Token::Comma) => continue,
                    Some(Token::Semicolon) | Some(Token::Eof) => break,
                    Some(tok) => return Err(ParseError::General(format!("Unexpected token in ORDER BY: {:?}", tok))),
                    None => return Err(ParseError::UnexpectedEnd),
                }
            }
            order_by = Some(cols);
        }

        Ok(Statement::Select 
        {
            columns,
            table,
            selection,
            order_by,
        })
    }

    fn parse_create_table(&mut self) -> Result<Statement, ParseError> 
    {
        self.expect_keyword(Keyword::Create)?;
        self.expect_keyword(Keyword::Table)?;

        let table_name = self.expect_identifier()?;

        self.expect(Token::LeftParentheses)?;

        let mut columns = Vec::new();

        loop 
        {
            let name = self.expect_identifier()?;

            let data_type = match self.advance() {
                Some(Token::Keyword(Keyword::Int)) => DBType::Int,
                Some(Token::Keyword(Keyword::Bool)) => DBType::Bool,
                Some(Token::Keyword(Keyword::Varchar)) => DBType::Varchar,
                Some(tok) => return Err(ParseError::General(format!("Unknown type: {:?}", tok))),
                None => return Err(ParseError::UnexpectedEnd),
            };

            let mut constraints = Vec::new();

            loop 
            {
                match self.peek() 
                {
                    Some(Token::Keyword(Keyword::Not)) => 
                    {
                        self.advance();
                        self.expect_keyword(Keyword::Null)?;
                        constraints.push(Constraint::NotNull);
                    }
                    Some(Token::Keyword(Keyword::Primary)) => 
                    {
                        self.advance();
                        self.expect_keyword(Keyword::Key)?;
                        constraints.push(Constraint::PrimaryKey);
                    }
                    Some(Token::Keyword(Keyword::Check)) => 
                    {
                        self.advance();
                        self.expect(Token::LeftParentheses)?;

                        let remaining = &self.tokens[self.position..];
                        let mut expr_parser = PrattParser::new(remaining);
                        let expr = expr_parser.parse_expression(1)
                            .map_err(ParseError::InvalidExpression)?;
                        self.position += expr_parser.position;

                        self.expect(Token::RightParentheses)?;
                        constraints.push(Constraint::Check(expr));
                    }
                    _ => break,
                }
            }

            columns.push(TableColumn 
            {
                name,
                data_type,
                constraints,
            });

            match self.advance() {
                Some(Token::Comma) => continue,
                Some(Token::RightParentheses) => break,
                Some(tok) => return Err(ParseError::General(format!("Expected ',' or ')', got: {:?}", tok))),
                None => return Err(ParseError::UnexpectedEnd),
            }
        }

        if let Some(Token::Semicolon) = self.peek() 
        {
            self.advance(); // Optional semicolon
        }

        Ok(Statement::CreateTable 
        {
            table_name,
            columns,
        })
    }
}
//editing to check 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_select_star() {
        let input = "SELECT * FROM users;";
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = tokenizer.next_token();
            if tok == Token::Eof {
                break;
            }
            tokens.push(tok);
        }
        let mut parser = SQLParser::new(&tokens);
        let stmt = parser.parse_statement().unwrap();

        match stmt {
            Statement::Select { columns, table, .. } => {
                assert_eq!(columns, vec!["*"]);
                assert_eq!(table, "users");
            }
            _ => panic!("Expected SELECT statement"),
        }
    }
}
