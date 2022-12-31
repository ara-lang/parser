use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::array::DictExpression;
use crate::tree::expression::array::TupleExpression;
use crate::tree::expression::array::VecExpression;
use crate::tree::expression::construct::ExitConstructExpression;
use crate::tree::expression::control_flow::MatchExpression;
use crate::tree::expression::function::AnonymousFunctionExpression;
use crate::tree::expression::function::ArrowFunctionExpression;
use crate::tree::expression::literal::Literal;
use crate::tree::expression::magic_constant::MagicConstant;
use crate::tree::expression::operator::ArithmeticOperationExpression;
use crate::tree::expression::operator::ArrayOperationExpression;
use crate::tree::expression::operator::AssignmentOperationExpression;
use crate::tree::expression::operator::AsyncOperationExpression;
use crate::tree::expression::operator::BitwiseOperationExpression;
use crate::tree::expression::operator::ClassOperationExpression;
use crate::tree::expression::operator::CoalesceOperationExpression;
use crate::tree::expression::operator::ComparisonOperationExpression;
use crate::tree::expression::operator::ExceptionOperationExpression;
use crate::tree::expression::operator::FunctionOperationExpression;
use crate::tree::expression::operator::GeneratorOperationExpression;
use crate::tree::expression::operator::LogicalOperationExpression;
use crate::tree::expression::operator::ObjectOperationExpression;
use crate::tree::expression::operator::StringOperationExpression;
use crate::tree::expression::operator::TernaryOperationExpression;
use crate::tree::expression::operator::TypeOperationExpression;
use crate::tree::identifier::Identifier;
use crate::tree::variable::Variable;
use crate::tree::Node;

pub mod argument;
pub mod array;
pub mod class;
pub mod construct;
pub mod control_flow;
pub mod function;
pub mod generic;
pub mod literal;
pub mod magic_constant;
pub mod operator;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum Expression {
    Parenthesized(ParenthesizedExpression),
    ExitConstruct(ExitConstructExpression),
    Literal(Literal),
    ArithmeticOperation(ArithmeticOperationExpression),
    AsyncOperation(AsyncOperationExpression),
    ArrayOperation(ArrayOperationExpression),
    AssignmentOperation(AssignmentOperationExpression),
    BitwiseOperation(BitwiseOperationExpression),
    ClassOperation(ClassOperationExpression),
    CoalesceOperation(CoalesceOperationExpression),
    ComparisonOperation(ComparisonOperationExpression),
    ExceptionOperation(ExceptionOperationExpression),
    FunctionOperation(FunctionOperationExpression),
    GeneratorOperation(GeneratorOperationExpression),
    LogicalOperation(LogicalOperationExpression),
    ObjectOperation(ObjectOperationExpression),
    StringOperation(StringOperationExpression),
    TypeOperation(TypeOperationExpression),
    TernaryOperation(TernaryOperationExpression),
    Identifier(Identifier),
    Variable(Variable),
    Match(MatchExpression),
    AnonymousFunction(AnonymousFunctionExpression),
    ArrowFunction(ArrowFunctionExpression),
    Vec(VecExpression),
    Dict(DictExpression),
    Tuple(TupleExpression),
    MagicConstant(MagicConstant),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ParenthesizedExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub expression: Box<Expression>,
    pub right_parenthesis: usize,
}

impl Node for ParenthesizedExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.left_parenthesis
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![self.expression.as_ref()]
    }
}

impl Node for Expression {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match self {
            Expression::Parenthesized(expression) => expression.initial_position(),
            Expression::ExitConstruct(expression) => expression.initial_position(),
            Expression::Literal(expression) => expression.initial_position(),
            Expression::ArithmeticOperation(expression) => expression.initial_position(),
            Expression::AsyncOperation(expression) => expression.initial_position(),
            Expression::ArrayOperation(expression) => expression.initial_position(),
            Expression::AssignmentOperation(expression) => expression.initial_position(),
            Expression::BitwiseOperation(expression) => expression.initial_position(),
            Expression::ClassOperation(expression) => expression.initial_position(),
            Expression::CoalesceOperation(expression) => expression.initial_position(),
            Expression::ComparisonOperation(expression) => expression.initial_position(),
            Expression::ExceptionOperation(expression) => expression.initial_position(),
            Expression::FunctionOperation(expression) => expression.initial_position(),
            Expression::GeneratorOperation(expression) => expression.initial_position(),
            Expression::LogicalOperation(expression) => expression.initial_position(),
            Expression::ObjectOperation(expression) => expression.initial_position(),
            Expression::StringOperation(expression) => expression.initial_position(),
            Expression::TypeOperation(expression) => expression.initial_position(),
            Expression::TernaryOperation(expression) => expression.initial_position(),
            Expression::Identifier(expression) => expression.initial_position(),
            Expression::Variable(expression) => expression.initial_position(),
            Expression::Match(expression) => expression.initial_position(),
            Expression::AnonymousFunction(expression) => expression.initial_position(),
            Expression::ArrowFunction(expression) => expression.initial_position(),
            Expression::Vec(expression) => expression.initial_position(),
            Expression::Dict(expression) => expression.initial_position(),
            Expression::Tuple(expression) => expression.initial_position(),
            Expression::MagicConstant(expression) => expression.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            Expression::Parenthesized(expression) => expression.final_position(),
            Expression::ExitConstruct(expression) => expression.final_position(),
            Expression::Literal(expression) => expression.final_position(),
            Expression::ArithmeticOperation(expression) => expression.final_position(),
            Expression::AsyncOperation(expression) => expression.final_position(),
            Expression::ArrayOperation(expression) => expression.final_position(),
            Expression::AssignmentOperation(expression) => expression.final_position(),
            Expression::BitwiseOperation(expression) => expression.final_position(),
            Expression::ClassOperation(expression) => expression.final_position(),
            Expression::CoalesceOperation(expression) => expression.final_position(),
            Expression::ComparisonOperation(expression) => expression.final_position(),
            Expression::ExceptionOperation(expression) => expression.final_position(),
            Expression::FunctionOperation(expression) => expression.final_position(),
            Expression::GeneratorOperation(expression) => expression.final_position(),
            Expression::LogicalOperation(expression) => expression.final_position(),
            Expression::ObjectOperation(expression) => expression.final_position(),
            Expression::StringOperation(expression) => expression.final_position(),
            Expression::TypeOperation(expression) => expression.final_position(),
            Expression::TernaryOperation(expression) => expression.final_position(),
            Expression::Identifier(expression) => expression.final_position(),
            Expression::Variable(expression) => expression.final_position(),
            Expression::Match(expression) => expression.final_position(),
            Expression::AnonymousFunction(expression) => expression.final_position(),
            Expression::ArrowFunction(expression) => expression.final_position(),
            Expression::Vec(expression) => expression.final_position(),
            Expression::Dict(expression) => expression.final_position(),
            Expression::Tuple(expression) => expression.final_position(),
            Expression::MagicConstant(expression) => expression.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Expression::Parenthesized(expression) => vec![expression],
            Expression::ExitConstruct(expression) => vec![expression],
            Expression::Literal(expression) => vec![expression],
            Expression::ArithmeticOperation(expression) => vec![expression],
            Expression::AsyncOperation(expression) => vec![expression],
            Expression::ArrayOperation(expression) => vec![expression],
            Expression::AssignmentOperation(expression) => vec![expression],
            Expression::BitwiseOperation(expression) => vec![expression],
            Expression::ClassOperation(expression) => vec![expression],
            Expression::CoalesceOperation(expression) => vec![expression],
            Expression::ComparisonOperation(expression) => vec![expression],
            Expression::ExceptionOperation(expression) => vec![expression],
            Expression::FunctionOperation(expression) => vec![expression],
            Expression::GeneratorOperation(expression) => vec![expression],
            Expression::LogicalOperation(expression) => vec![expression],
            Expression::ObjectOperation(expression) => vec![expression],
            Expression::StringOperation(expression) => vec![expression],
            Expression::TypeOperation(expression) => vec![expression],
            Expression::TernaryOperation(expression) => vec![expression],
            Expression::Identifier(expression) => vec![expression],
            Expression::Variable(expression) => vec![expression],
            Expression::Match(expression) => vec![expression],
            Expression::AnonymousFunction(expression) => vec![expression],
            Expression::ArrowFunction(expression) => vec![expression],
            Expression::Vec(expression) => vec![expression],
            Expression::Dict(expression) => vec![expression],
            Expression::Tuple(expression) => vec![expression],
            Expression::MagicConstant(expression) => vec![expression],
        }
    }
}
