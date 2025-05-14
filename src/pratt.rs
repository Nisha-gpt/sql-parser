//Created by: Nisha Murali (Requested not to clone / re-use the code)
//pratt.rs (Parses the expressions like a>b AND c<d(WHERE, ORDER))
use crate::tokenizer::{Token, Keyword};
use crate::ast::{Expression, BinaryOperator, UnaryOperator};

pub struct PrattParser<'a> 
{
    pub tokens: &'a [Token],
    pub position: usize,
}
impl<'a> PrattParser<'a> 
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

    fn get_precedence(token: &Token) -> u8 
    {
        match token 
        {
            Token::Keyword(Keyword::Or) => 1,
            Token::Keyword(Keyword::And) => 2,
            Token::Equal | Token::NotEqual => 3,
            Token::GreaterThan | Token::GreaterThanOrEqual |
            Token::LessThan | Token::LessThanOrEqual => 4,
            Token::Plus | Token::Minus => 5,
            Token::Multiply | Token::Divide => 6,
            _ => 0,
        }
    }

    pub fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, String> 
    {
        let mut left = match self.advance() 
        {
            Some(Token::Identifier(name)) => Expression::Identifier(name.clone()),
            Some(Token::Number(n)) => Expression::Number(*n),
            Some(Token::String(s)) => Expression::String(s.clone()),
            Some(Token::Keyword(Keyword::True)) => Expression::Boolean(true),
            Some(Token::Keyword(Keyword::False)) => Expression::Boolean(false),
            Some(Token::Keyword(Keyword::Not)) => 
            {
                let expr = self.parse_expression(6)?;
                Expression::UnaryOperation {
                    operator: UnaryOperator::Not,
                    operand: Box::new(expr),
                }
            }
            Some(Token::Minus) => 
            {
                let expr = self.parse_expression(6)?;
                Expression::UnaryOperation 
                {
                    operator: UnaryOperator::Negate,
                    operand: Box::new(expr),
                }
            }
            Some(Token::LeftParentheses) => 
            {
                let expr = self.parse_expression(1)?;
                match self.advance() 
                {
                    Some(Token::RightParentheses) => Expression::Grouped(Box::new(expr)),
                    _ => return Err("Expected ')'".to_string()),
                }
            }
            Some(t) => return Err(format!("Unexpected token: {:?}", t)),
            None => return Err("Unexpected end of input".to_string()),
        };

        loop {
            let op = match self.peek() 
            {
                Some(tok) if Self::get_precedence(tok) >= min_precedence => tok.clone(),
                _ => break,
            };

            let precedence = Self::get_precedence(&op);
            self.advance();

            let right = self.parse_expression(precedence + 1)?;

            let operator = match op 
            {
                Token::Equal => BinaryOperator::Equals,
                Token::NotEqual => BinaryOperator::NotEquals,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                Token::LessThan => BinaryOperator::LessThan,
                Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                Token::Keyword(Keyword::And) => BinaryOperator::And,
                Token::Keyword(Keyword::Or) => BinaryOperator::Or,
                _ => return Err(format!("Unknown operator {:?}", op)),
            };

            left = Expression::BinaryOperation 
            {
                left_operand: Box::new(left),
                operator,
                right_operand: Box::new(right),
            };
        }

        Ok(left)
    }
}
//editing
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::{Tokenizer, Token};

    #[test]
    fn test_simple_expression() {
        let tokens = {
            let mut tokenizer = Tokenizer::new("age >= 18");
            let mut tokens = Vec::new();
            loop {
                let tok = tokenizer.next_token();
                tokens.push(tok.clone());
                if tok == Token::Eof {
                    break;
                }
            }
            tokens
        };

        let mut parser = PrattParser::new(&tokens);
        let expr = parser.parse_expression(1).unwrap();
        match expr {
            Expression::BinaryOperation { .. } => {} // success
            _ => panic!("Expected binary operation"),
        }
    }
}
