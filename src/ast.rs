//Created by: Nisha Murali (Requested not to clone / re-use the code)
//asst.rs (Structure and enums:Token, Statement, Expression, etc.)
use crate::tokenizer::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression 
{
    Identifier(String),
    Number(u64),
    String(String),
    UnaryOperation 
    {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    BinaryOperation 
    {
        left_operand: Box<Expression>,
        operator: BinaryOperator,
        right_operand: Box<Expression>,
    },
    Boolean(bool),
    Null,
    Grouped(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator 
{
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator 
{
    Not,
    Negate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement 
{
    Select 
    {
        columns: Vec<String>,
        table: String,
        selection: Option<Expression>,
        order_by: Option<Vec<String>>,
    },
    CreateTable 
    {
        table_name: String,
        columns: Vec<TableColumn>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableColumn 
{
    pub name: String,
    pub data_type: DBType,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DBType 
{
    Int,
    Bool,
    Varchar,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint 
{
    NotNull,
    PrimaryKey,
    Check(Expression),
}
