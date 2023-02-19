use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::argument::ArgumentExpression;
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
use crate::tree::expression::operator::FunctionalOperationExpression;
use crate::tree::expression::operator::GeneratorOperationExpression;
use crate::tree::expression::operator::LogicalOperationExpression;
use crate::tree::expression::operator::ObjectOperationExpression;
use crate::tree::expression::operator::RangeOperationExpression;
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum Expression {
    Parenthesized(ParenthesizedExpression),
    ExitConstruct(ExitConstructExpression),
    Literal(Literal),
    FunctionalOperation(FunctionalOperationExpression),
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
    RangeOperation(RangeOperationExpression),
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

    fn get_description(&self) -> String {
        "parenthesized expression".to_string()
    }
}

impl Expression {
    /// Return true if the expression is a constant expression.
    ///
    /// If `initilization` is true, the expression is allowed to contain a class initialization expression.
    pub fn is_constant(&self, initilization: bool) -> bool {
        match &self {
            Self::Literal(_) => true,
            Self::Identifier(_) => true,
            Self::MagicConstant(_) => true,
            Self::Parenthesized(expression) => expression.expression.is_constant(initilization),
            Self::ArithmeticOperation(expression) => match &expression {
                ArithmeticOperationExpression::Addition { left, right, .. }
                | ArithmeticOperationExpression::Subtraction { left, right, .. }
                | ArithmeticOperationExpression::Multiplication { left, right, .. }
                | ArithmeticOperationExpression::Division { left, right, .. }
                | ArithmeticOperationExpression::Modulo { left, right, .. }
                | ArithmeticOperationExpression::Exponentiation { left, right, .. } => {
                    left.is_constant(initilization) && right.is_constant(initilization)
                }
                ArithmeticOperationExpression::Negative { right, .. }
                | ArithmeticOperationExpression::Positive { right, .. } => {
                    right.is_constant(initilization)
                }
                ArithmeticOperationExpression::PreIncrement { .. }
                | ArithmeticOperationExpression::PreDecrement { .. }
                | ArithmeticOperationExpression::PostIncrement { .. }
                | ArithmeticOperationExpression::PostDecrement { .. } => false,
            },
            Self::ArrayOperation(ArrayOperationExpression::Access { array, index, .. }) => {
                array.is_constant(initilization) && index.is_constant(initilization)
            }
            Self::BitwiseOperation(expression) => match &expression {
                BitwiseOperationExpression::And { left, right, .. }
                | BitwiseOperationExpression::Or { left, right, .. }
                | BitwiseOperationExpression::Xor { left, right, .. }
                | BitwiseOperationExpression::LeftShift { left, right, .. }
                | BitwiseOperationExpression::RightShift { left, right, .. } => {
                    left.is_constant(initilization) && right.is_constant(initilization)
                }
                BitwiseOperationExpression::Not { right, .. } => right.is_constant(initilization),
            },
            Self::ClassOperation(expression) => match &expression {
                ClassOperationExpression::Initialization {
                    class, arguments, ..
                } if initilization => match class {
                    operator::ClassOperationInitializationClassExpression::Variable(_) => false,
                    operator::ClassOperationInitializationClassExpression::Identifier(_) => {
                        arguments
                            .arguments
                            .inner
                            .iter()
                            .all(|argument| match argument {
                                ArgumentExpression::Value { value, .. } => {
                                    value.is_constant(initilization)
                                }
                                ArgumentExpression::Named { value, .. } => {
                                    value.is_constant(initilization)
                                }
                                // spreading arguments cannot be used in constant expressions
                                _ => false,
                            })
                    }
                },
                ClassOperationExpression::ConstantFetch { class, .. } => {
                    class.is_constant(initilization)
                }
                _ => false,
            },
            Self::CoalesceOperation(CoalesceOperationExpression::Coalesce {
                left, right, ..
            }) => left.is_constant(initilization) && right.is_constant(initilization),
            Self::ComparisonOperation(expression) => match &expression {
                ComparisonOperationExpression::Equal { left, right, .. }
                | ComparisonOperationExpression::NotEqual { left, right, .. }
                | ComparisonOperationExpression::Identical { left, right, .. }
                | ComparisonOperationExpression::NotIdentical { left, right, .. }
                | ComparisonOperationExpression::LessThan { left, right, .. }
                | ComparisonOperationExpression::LessThanOrEqual { left, right, .. }
                | ComparisonOperationExpression::GreaterThan { left, right, .. }
                | ComparisonOperationExpression::GreaterThanOrEqual { left, right, .. } => {
                    left.is_constant(initilization) && right.is_constant(initilization)
                }
                ComparisonOperationExpression::Spaceship { left, right, .. } => {
                    left.is_constant(initilization) && right.is_constant(initilization)
                }
            },
            Self::LogicalOperation(expression) => match &expression {
                LogicalOperationExpression::And { left, right, .. }
                | LogicalOperationExpression::Or { left, right, .. } => {
                    left.is_constant(initilization) && right.is_constant(initilization)
                }
                LogicalOperationExpression::Not { right, .. } => right.is_constant(initilization),
            },
            Self::StringOperation(StringOperationExpression::Concat { left, right, .. }) => {
                left.is_constant(initilization) && right.is_constant(initilization)
            }
            Self::TernaryOperation(expression) => match &expression {
                TernaryOperationExpression::Ternary {
                    condition,
                    if_true,
                    if_false,
                    ..
                } => {
                    condition.is_constant(initilization)
                        && if_true.is_constant(initilization)
                        && if_false.is_constant(initilization)
                }
                TernaryOperationExpression::ShortTernary {
                    condition,
                    if_false,
                    ..
                } => condition.is_constant(initilization) && if_false.is_constant(initilization),
                TernaryOperationExpression::ImplicitShortTernary {
                    condition,
                    if_false,
                    ..
                } => condition.is_constant(initilization) && if_false.is_constant(initilization),
            },
            Self::Vec(expression) => expression
                .elements
                .inner
                .iter()
                .all(|element| element.value.is_constant(initilization)),
            Self::Dict(expression) => expression.elements.inner.iter().all(|element| {
                element.value.is_constant(initilization) && element.key.is_constant(initilization)
            }),
            Self::Tuple(expression) => expression
                .elements
                .inner
                .iter()
                .all(|element| element.is_constant(initilization)),
            _ => false,
        }
    }

    /// Return true if the expression is writable
    pub fn is_writable(&self) -> bool {
        match &self {
            Self::Variable(_)
            | Self::ArrayOperation(ArrayOperationExpression::Push { .. })
            | Self::ArrayOperation(ArrayOperationExpression::Access { .. })
            | Self::ObjectOperation(ObjectOperationExpression::PropertyFetch { .. })
            | Self::ClassOperation(ClassOperationExpression::StaticPropertyFetch { .. }) => true,
            Self::Tuple(TupleExpression { elements, .. }) => {
                elements.inner.iter().all(|element| element.is_writable())
            }
            _ => false,
        }
    }

    /// Return true if the expression is readable
    pub fn is_readable(&self) -> bool {
        match &self {
            Self::AssignmentOperation(..)
            | Self::ExitConstruct(..)
            | Self::ExceptionOperation(ExceptionOperationExpression::Throw { .. })
            | Self::ArrayOperation(ArrayOperationExpression::Push { .. }) => false,
            Self::AsyncOperation(AsyncOperationExpression::Concurrently {
                expressions, ..
            }) => expressions
                .inner
                .iter()
                .all(|expression| expression.is_readable()),
            _ => true,
        }
    }
}

impl Node for Expression {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Parenthesized(expression) => expression.initial_position(),
            Self::ExitConstruct(expression) => expression.initial_position(),
            Self::Literal(expression) => expression.initial_position(),
            Self::ArithmeticOperation(expression) => expression.initial_position(),
            Self::AsyncOperation(expression) => expression.initial_position(),
            Self::ArrayOperation(expression) => expression.initial_position(),
            Self::AssignmentOperation(expression) => expression.initial_position(),
            Self::BitwiseOperation(expression) => expression.initial_position(),
            Self::ClassOperation(expression) => expression.initial_position(),
            Self::CoalesceOperation(expression) => expression.initial_position(),
            Self::ComparisonOperation(expression) => expression.initial_position(),
            Self::ExceptionOperation(expression) => expression.initial_position(),
            Self::FunctionOperation(expression) => expression.initial_position(),
            Self::GeneratorOperation(expression) => expression.initial_position(),
            Self::LogicalOperation(expression) => expression.initial_position(),
            Self::ObjectOperation(expression) => expression.initial_position(),
            Self::RangeOperation(expression) => expression.initial_position(),
            Self::StringOperation(expression) => expression.initial_position(),
            Self::TypeOperation(expression) => expression.initial_position(),
            Self::TernaryOperation(expression) => expression.initial_position(),
            Self::Identifier(expression) => expression.initial_position(),
            Self::Variable(expression) => expression.initial_position(),
            Self::Match(expression) => expression.initial_position(),
            Self::AnonymousFunction(expression) => expression.initial_position(),
            Self::ArrowFunction(expression) => expression.initial_position(),
            Self::Vec(expression) => expression.initial_position(),
            Self::Dict(expression) => expression.initial_position(),
            Self::Tuple(expression) => expression.initial_position(),
            Self::MagicConstant(expression) => expression.initial_position(),
            Self::FunctionalOperation(expression) => expression.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Parenthesized(expression) => expression.final_position(),
            Self::ExitConstruct(expression) => expression.final_position(),
            Self::Literal(expression) => expression.final_position(),
            Self::ArithmeticOperation(expression) => expression.final_position(),
            Self::AsyncOperation(expression) => expression.final_position(),
            Self::ArrayOperation(expression) => expression.final_position(),
            Self::AssignmentOperation(expression) => expression.final_position(),
            Self::BitwiseOperation(expression) => expression.final_position(),
            Self::ClassOperation(expression) => expression.final_position(),
            Self::CoalesceOperation(expression) => expression.final_position(),
            Self::ComparisonOperation(expression) => expression.final_position(),
            Self::ExceptionOperation(expression) => expression.final_position(),
            Self::FunctionOperation(expression) => expression.final_position(),
            Self::GeneratorOperation(expression) => expression.final_position(),
            Self::LogicalOperation(expression) => expression.final_position(),
            Self::ObjectOperation(expression) => expression.final_position(),
            Self::RangeOperation(expression) => expression.final_position(),
            Self::StringOperation(expression) => expression.final_position(),
            Self::TypeOperation(expression) => expression.final_position(),
            Self::TernaryOperation(expression) => expression.final_position(),
            Self::Identifier(expression) => expression.final_position(),
            Self::Variable(expression) => expression.final_position(),
            Self::Match(expression) => expression.final_position(),
            Self::AnonymousFunction(expression) => expression.final_position(),
            Self::ArrowFunction(expression) => expression.final_position(),
            Self::Vec(expression) => expression.final_position(),
            Self::Dict(expression) => expression.final_position(),
            Self::Tuple(expression) => expression.final_position(),
            Self::MagicConstant(expression) => expression.final_position(),
            Self::FunctionalOperation(expression) => expression.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Parenthesized(expression) => vec![expression],
            Self::ExitConstruct(expression) => vec![expression],
            Self::Literal(expression) => vec![expression],
            Self::ArithmeticOperation(expression) => vec![expression],
            Self::AsyncOperation(expression) => vec![expression],
            Self::ArrayOperation(expression) => vec![expression],
            Self::AssignmentOperation(expression) => vec![expression],
            Self::BitwiseOperation(expression) => vec![expression],
            Self::ClassOperation(expression) => vec![expression],
            Self::CoalesceOperation(expression) => vec![expression],
            Self::ComparisonOperation(expression) => vec![expression],
            Self::ExceptionOperation(expression) => vec![expression],
            Self::FunctionOperation(expression) => vec![expression],
            Self::GeneratorOperation(expression) => vec![expression],
            Self::LogicalOperation(expression) => vec![expression],
            Self::ObjectOperation(expression) => vec![expression],
            Self::RangeOperation(expression) => vec![expression],
            Self::StringOperation(expression) => vec![expression],
            Self::TypeOperation(expression) => vec![expression],
            Self::TernaryOperation(expression) => vec![expression],
            Self::Identifier(expression) => vec![expression],
            Self::Variable(expression) => vec![expression],
            Self::Match(expression) => vec![expression],
            Self::AnonymousFunction(expression) => vec![expression],
            Self::ArrowFunction(expression) => vec![expression],
            Self::Vec(expression) => vec![expression],
            Self::Dict(expression) => vec![expression],
            Self::Tuple(expression) => vec![expression],
            Self::MagicConstant(expression) => vec![expression],
            Self::FunctionalOperation(expression) => vec![expression],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Parenthesized(expression) => expression.get_description(),
            Self::ExitConstruct(expression) => expression.get_description(),
            Self::Literal(expression) => expression.get_description(),
            Self::ArithmeticOperation(expression) => expression.get_description(),
            Self::AsyncOperation(expression) => expression.get_description(),
            Self::ArrayOperation(expression) => expression.get_description(),
            Self::AssignmentOperation(expression) => expression.get_description(),
            Self::BitwiseOperation(expression) => expression.get_description(),
            Self::ClassOperation(expression) => expression.get_description(),
            Self::CoalesceOperation(expression) => expression.get_description(),
            Self::ComparisonOperation(expression) => expression.get_description(),
            Self::ExceptionOperation(expression) => expression.get_description(),
            Self::FunctionOperation(expression) => expression.get_description(),
            Self::GeneratorOperation(expression) => expression.get_description(),
            Self::LogicalOperation(expression) => expression.get_description(),
            Self::ObjectOperation(expression) => expression.get_description(),
            Self::RangeOperation(expression) => expression.get_description(),
            Self::StringOperation(expression) => expression.get_description(),
            Self::TypeOperation(expression) => expression.get_description(),
            Self::TernaryOperation(expression) => expression.get_description(),
            Self::Identifier(expression) => expression.get_description(),
            Self::Variable(expression) => expression.get_description(),
            Self::Match(expression) => expression.get_description(),
            Self::AnonymousFunction(expression) => expression.get_description(),
            Self::ArrowFunction(expression) => expression.get_description(),
            Self::Vec(expression) => expression.get_description(),
            Self::Dict(expression) => expression.get_description(),
            Self::Tuple(expression) => expression.get_description(),
            Self::MagicConstant(expression) => expression.get_description(),
            Self::FunctionalOperation(expression) => expression.get_description(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Parenthesized(expression) => write!(f, "{}", expression),
            Self::ExitConstruct(expression) => write!(f, "{}", expression),
            Self::Literal(expression) => write!(f, "{}", expression),
            Self::ArithmeticOperation(expression) => write!(f, "{}", expression),
            Self::AsyncOperation(expression) => write!(f, "{}", expression),
            Self::ArrayOperation(expression) => write!(f, "{}", expression),
            Self::AssignmentOperation(expression) => write!(f, "{}", expression),
            Self::BitwiseOperation(expression) => write!(f, "{}", expression),
            Self::ClassOperation(expression) => write!(f, "{}", expression),
            Self::CoalesceOperation(expression) => write!(f, "{}", expression),
            Self::ComparisonOperation(expression) => write!(f, "{}", expression),
            Self::ExceptionOperation(expression) => write!(f, "{}", expression),
            Self::FunctionOperation(expression) => write!(f, "{}", expression),
            Self::GeneratorOperation(expression) => write!(f, "{}", expression),
            Self::LogicalOperation(expression) => write!(f, "{}", expression),
            Self::ObjectOperation(expression) => write!(f, "{}", expression),
            Self::RangeOperation(expression) => write!(f, "{}", expression),
            Self::StringOperation(expression) => write!(f, "{}", expression),
            Self::TypeOperation(expression) => write!(f, "{}", expression),
            Self::TernaryOperation(expression) => write!(f, "{}", expression),
            Self::Identifier(expression) => write!(f, "{}", expression),
            Self::Variable(expression) => write!(f, "{}", expression),
            Self::Match(expression) => write!(f, "{}", expression),
            Self::AnonymousFunction(expression) => write!(f, "{}", expression),
            Self::ArrowFunction(expression) => write!(f, "{}", expression),
            Self::Vec(expression) => write!(f, "{}", expression),
            Self::Dict(expression) => write!(f, "{}", expression),
            Self::Tuple(expression) => write!(f, "{}", expression),
            Self::MagicConstant(expression) => write!(f, "{}", expression),
            Self::FunctionalOperation(expression) => write!(f, "{}", expression),
        }
    }
}

impl std::fmt::Display for ParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.expression)
    }
}
