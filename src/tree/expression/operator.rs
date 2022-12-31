use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::expression::argument::ArgumentListExpression;
use crate::tree::expression::argument::ArgumentPlaceholderExpression;
use crate::tree::expression::class::AnonymousClassExpression;
use crate::tree::expression::generic::GenericGroupExpression;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ArithmeticOperationExpression {
    Addition {
        comments: CommentGroup,
        left: Box<Expression>,
        plus: Span,
        right: Box<Expression>,
    },
    Subtraction {
        comments: CommentGroup,
        left: Box<Expression>,
        minus: Span,
        right: Box<Expression>,
    },
    Multiplication {
        comments: CommentGroup,
        left: Box<Expression>,
        asterisk: Span,
        right: Box<Expression>,
    },
    Division {
        comments: CommentGroup,
        left: Box<Expression>,
        slash: Span,
        right: Box<Expression>,
    },
    Modulo {
        comments: CommentGroup,
        left: Box<Expression>,
        percent: Span,
        right: Box<Expression>,
    },
    Exponentiation {
        comments: CommentGroup,
        left: Box<Expression>,
        pow: Span,
        right: Box<Expression>,
    },
    Negative {
        comments: CommentGroup,
        minus: Span,
        right: Box<Expression>,
    },
    Positive {
        comments: CommentGroup,
        plus: Span,
        right: Box<Expression>,
    },
    PreIncrement {
        comments: CommentGroup,
        increment: Span,
        right: Box<Expression>,
    },
    PostIncrement {
        left: Box<Expression>,
        increment: Span,
    },
    PreDecrement {
        comments: CommentGroup,
        decrement: Span,
        right: Box<Expression>,
    },
    PostDecrement {
        left: Box<Expression>,
        decrement: Span,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum AssignmentOperationExpression {
    Assignment {
        comments: CommentGroup,
        left: Box<Expression>,
        equals: Span,
        right: Box<Expression>,
    },
    Addition {
        comments: CommentGroup,
        left: Box<Expression>,
        plus_equals: Span,
        right: Box<Expression>,
    },
    Subtraction {
        comments: CommentGroup,
        left: Box<Expression>,
        minus_equals: Span,
        right: Box<Expression>,
    },
    Multiplication {
        comments: CommentGroup,
        left: Box<Expression>,
        asterisk_equals: Span,
        right: Box<Expression>,
    },
    Division {
        comments: CommentGroup,
        left: Box<Expression>,
        slash_equals: Span,
        right: Box<Expression>,
    },
    Modulo {
        comments: CommentGroup,
        left: Box<Expression>,
        percent_equals: Span,
        right: Box<Expression>,
    },
    Exponentiation {
        comments: CommentGroup,
        left: Box<Expression>,
        pow_equals: Span,
        right: Box<Expression>,
    },
    Concat {
        comments: CommentGroup,
        left: Box<Expression>,
        dot_equals: Span,
        right: Box<Expression>,
    },
    BitwiseAnd {
        comments: CommentGroup,
        left: Box<Expression>,
        ampersand_equals: Span,
        right: Box<Expression>,
    },
    BitwiseOr {
        comments: CommentGroup,
        left: Box<Expression>,
        pipe_equals: Span,
        right: Box<Expression>,
    },
    BitwiseXor {
        comments: CommentGroup,
        left: Box<Expression>,
        caret_equals: Span,
        right: Box<Expression>,
    },
    LeftShift {
        comments: CommentGroup,
        left: Box<Expression>,
        left_shift_equals: Span,
        right: Box<Expression>,
    },
    RightShift {
        comments: CommentGroup,
        left: Box<Expression>,
        right_shift_equals: Span,
        right: Box<Expression>,
    },
    Coalesce {
        comments: CommentGroup,
        left: Box<Expression>,
        coalesce_equals: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum BitwiseOperationExpression {
    And {
        comments: CommentGroup,
        left: Box<Expression>,
        and: Span,
        right: Box<Expression>,
    },
    Or {
        comments: CommentGroup,
        left: Box<Expression>,
        or: Span,
        right: Box<Expression>,
    },
    Xor {
        comments: CommentGroup,
        left: Box<Expression>,
        xor: Span,
        right: Box<Expression>,
    },
    LeftShift {
        comments: CommentGroup,
        left: Box<Expression>,
        left_shift: Span,
        right: Box<Expression>,
    },
    RightShift {
        comments: CommentGroup,
        left: Box<Expression>,
        right_shift: Span,
        right: Box<Expression>,
    },
    Not {
        comments: CommentGroup,
        not: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ComparisonOperationExpression {
    Equal {
        comments: CommentGroup,
        left: Box<Expression>,
        double_equals: Span,
        right: Box<Expression>,
    },
    Identical {
        comments: CommentGroup,
        left: Box<Expression>,
        triple_equals: Span,
        right: Box<Expression>,
    },
    NotEqual {
        comments: CommentGroup,
        left: Box<Expression>,
        bang_equals: Span,
        right: Box<Expression>,
    },
    AngledNotEqual {
        comments: CommentGroup,
        left: Box<Expression>,
        angled_left_right: Span,
        right: Box<Expression>,
    },
    NotIdentical {
        comments: CommentGroup,
        left: Box<Expression>,
        bang_double_equals: Span,
        right: Box<Expression>,
    },
    LessThan {
        comments: CommentGroup,
        left: Box<Expression>,
        less_than: Span,
        right: Box<Expression>,
    },
    GreaterThan {
        comments: CommentGroup,
        left: Box<Expression>,
        greater_than: Span,
        right: Box<Expression>,
    },
    LessThanOrEqual {
        comments: CommentGroup,
        left: Box<Expression>,
        less_than_equals: Span,
        right: Box<Expression>,
    },
    GreaterThanOrEqual {
        comments: CommentGroup,
        left: Box<Expression>,
        greater_than_equals: Span,
        right: Box<Expression>,
    },
    Spaceship {
        comments: CommentGroup,
        left: Box<Expression>,
        spaceship: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum LogicalOperationExpression {
    And {
        comments: CommentGroup,
        left: Box<Expression>,
        double_ampersand: Span,
        right: Box<Expression>,
    },
    Or {
        comments: CommentGroup,
        left: Box<Expression>,
        double_pipe: Span,
        right: Box<Expression>,
    },
    Not {
        comments: CommentGroup,
        bang: Span,
        right: Box<Expression>,
    },
    LogicalAnd {
        comments: CommentGroup,
        left: Box<Expression>,
        and: Span,
        right: Box<Expression>,
    },
    LogicalOr {
        comments: CommentGroup,
        left: Box<Expression>,
        or: Span,
        right: Box<Expression>,
    },
    LogicalXor {
        comments: CommentGroup,
        left: Box<Expression>,
        xor: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum StringOperationExpression {
    Concat {
        comments: CommentGroup,
        left: Box<Expression>,
        dot: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ArrayOperationExpression {
    Access {
        comments: CommentGroup,
        array: Box<Expression>,
        left_bracket: Span,
        index: Box<Expression>,
        right_bracket: Span,
    },
    Push {
        comments: CommentGroup,
        array: Box<Expression>,
        left_bracket: Span,
        right_bracket: Span,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum CoalesceOperationExpression {
    Coalesce {
        comments: CommentGroup,
        left: Box<Expression>,
        double_question: Span,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TernaryOperationExpression {
    Ternary {
        comments: CommentGroup,
        condition: Box<Expression>,
        question: Span,
        if_true: Box<Expression>,
        colon: Span,
        if_false: Box<Expression>,
    },
    ImplicitShortTernary {
        comments: CommentGroup,
        condition: Box<Expression>,
        question: Span,
        colon: Span,
        if_false: Box<Expression>,
    },
    ShortTernary {
        comments: CommentGroup,
        condition: Box<Expression>,
        question_colon: Span,
        if_false: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TypeOperationExpression {
    Instanceof {
        comments: CommentGroup,
        left: Box<Expression>,
        instanceof: Span,
        right: Identifier,
    },
    Is {
        comments: CommentGroup,
        left: Box<Expression>,
        is: Span,
        right: TypeDefinition,
    },
    As {
        comments: CommentGroup,
        left: Box<Expression>,
        r#as: Span,
        right: TypeDefinition,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum GeneratorOperationExpression {
    Yield {
        comments: CommentGroup,
        r#yield: Span,
    },
    YieldValue {
        comments: CommentGroup,
        r#yield: Span,
        value: Box<Expression>,
    },
    YieldKeyValue {
        comments: CommentGroup,
        r#yield: Span,
        key: Box<Expression>,
        double_arrow: Span,
        value: Box<Expression>,
    },
    YieldFrom {
        comments: CommentGroup,
        r#yield: Span,
        from: Span,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ExceptionOperationExpression {
    Throw {
        comments: CommentGroup,
        r#throw: Span,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ObjectOperationExpression {
    Clone {
        comments: CommentGroup,
        clone: Span,
        object: Box<Expression>,
    },
    MethodCall {
        comments: CommentGroup,
        object: Box<Expression>,
        arrow: Span,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    NullsafeMethodCall {
        comments: CommentGroup,
        object: Box<Expression>,
        question_arrow: Span,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    MethodClosureCreation {
        comments: CommentGroup,
        object: Box<Expression>,
        arrow: Span,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        placeholder: ArgumentPlaceholderExpression,
    },
    PropertyFetch {
        comments: CommentGroup,
        object: Box<Expression>,
        arrow: Span,
        property: Identifier,
    },
    NullsafePropertyFetch {
        comments: CommentGroup,
        object: Box<Expression>,
        question_arrow: Span,
        property: Identifier,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ClassOperationExpression {
    Initialization {
        comments: CommentGroup,
        new: Span,
        class: Identifier,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    AnonymousInitialization {
        comments: CommentGroup,
        new: Span,
        class: AnonymousClassExpression,
    },
    StaticMethodCall {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: Span,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    StaticMethodClosureCreation {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: Span,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        placeholder: ArgumentPlaceholderExpression,
    },
    StaticPropertyFetch {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: Span,
        property: Variable,
    },
    ConstantFetch {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: Span,
        constant: Identifier,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum FunctionOperationExpression {
    Call {
        comments: CommentGroup,
        function: Box<Expression>,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    ClosureCreation {
        comments: CommentGroup,
        function: Box<Expression>,
        generics: Option<GenericGroupExpression>,
        placeholder: ArgumentPlaceholderExpression,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum AsyncOperationExpression {
    Async {
        comments: CommentGroup,
        r#async: Span,
        expression: Box<Expression>,
    },
    Await {
        comments: CommentGroup,
        r#await: Span,
        expression: Box<Expression>,
    },
    Concurrently {
        comments: CommentGroup,
        concurrently: Span,
        left_brace: Span,
        expressions: CommaSeparated<Expression>,
        right_brace: Span,
    },
}

impl Node for ArithmeticOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ArithmeticOperationExpression::Addition { comments, .. } => Some(comments),
            ArithmeticOperationExpression::Subtraction { comments, .. } => Some(comments),
            ArithmeticOperationExpression::Multiplication { comments, .. } => Some(comments),
            ArithmeticOperationExpression::Division { comments, .. } => Some(comments),
            ArithmeticOperationExpression::Modulo { comments, .. } => Some(comments),
            ArithmeticOperationExpression::Exponentiation { comments, .. } => Some(comments),
            ArithmeticOperationExpression::Negative { comments, .. } => Some(comments),
            ArithmeticOperationExpression::Positive { comments, .. } => Some(comments),
            ArithmeticOperationExpression::PreIncrement { comments, .. } => Some(comments),
            ArithmeticOperationExpression::PreDecrement { comments, .. } => Some(comments),
            ArithmeticOperationExpression::PostIncrement { .. } => None,
            ArithmeticOperationExpression::PostDecrement { .. } => None,
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ArithmeticOperationExpression::Addition { left, .. } => left.initial_position(),
            ArithmeticOperationExpression::Subtraction { left, .. } => left.initial_position(),
            ArithmeticOperationExpression::Multiplication { left, .. } => left.initial_position(),
            ArithmeticOperationExpression::Division { left, .. } => left.initial_position(),
            ArithmeticOperationExpression::Modulo { left, .. } => left.initial_position(),
            ArithmeticOperationExpression::Exponentiation { left, .. } => left.initial_position(),
            ArithmeticOperationExpression::Negative { minus, .. } => minus.position,
            ArithmeticOperationExpression::Positive { plus, .. } => plus.position,
            ArithmeticOperationExpression::PreIncrement { increment, .. } => increment.position,
            ArithmeticOperationExpression::PreDecrement { decrement, .. } => decrement.position,
            ArithmeticOperationExpression::PostIncrement { left, .. } => left.initial_position(),
            ArithmeticOperationExpression::PostDecrement { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ArithmeticOperationExpression::Addition { right, .. } => right.final_position(),
            ArithmeticOperationExpression::Subtraction { right, .. } => right.final_position(),
            ArithmeticOperationExpression::Multiplication { right, .. } => right.final_position(),
            ArithmeticOperationExpression::Division { right, .. } => right.final_position(),
            ArithmeticOperationExpression::Modulo { right, .. } => right.final_position(),
            ArithmeticOperationExpression::Exponentiation { right, .. } => right.final_position(),
            ArithmeticOperationExpression::Negative { right, .. } => right.final_position(),
            ArithmeticOperationExpression::Positive { right, .. } => right.final_position(),
            ArithmeticOperationExpression::PreIncrement { right, .. } => right.final_position(),
            ArithmeticOperationExpression::PreDecrement { right, .. } => right.final_position(),
            ArithmeticOperationExpression::PostIncrement { increment, .. } => increment.position,
            ArithmeticOperationExpression::PostDecrement { decrement, .. } => decrement.position,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ArithmeticOperationExpression::Addition { left, right, .. }
            | ArithmeticOperationExpression::Subtraction { left, right, .. }
            | ArithmeticOperationExpression::Multiplication { left, right, .. }
            | ArithmeticOperationExpression::Division { left, right, .. }
            | ArithmeticOperationExpression::Modulo { left, right, .. }
            | ArithmeticOperationExpression::Exponentiation { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
            ArithmeticOperationExpression::Negative { right, .. }
            | ArithmeticOperationExpression::Positive { right, .. }
            | ArithmeticOperationExpression::PreIncrement { right, .. }
            | ArithmeticOperationExpression::PreDecrement { right, .. } => vec![right.as_ref()],
            ArithmeticOperationExpression::PostIncrement { left, .. }
            | ArithmeticOperationExpression::PostDecrement { left, .. } => {
                vec![left.as_ref()]
            }
        }
    }
}

impl Node for AssignmentOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            AssignmentOperationExpression::Assignment { comments, .. } => Some(comments),
            AssignmentOperationExpression::Addition { comments, .. } => Some(comments),
            AssignmentOperationExpression::Subtraction { comments, .. } => Some(comments),
            AssignmentOperationExpression::Multiplication { comments, .. } => Some(comments),
            AssignmentOperationExpression::Division { comments, .. } => Some(comments),
            AssignmentOperationExpression::Modulo { comments, .. } => Some(comments),
            AssignmentOperationExpression::Exponentiation { comments, .. } => Some(comments),
            AssignmentOperationExpression::BitwiseAnd { comments, .. } => Some(comments),
            AssignmentOperationExpression::BitwiseOr { comments, .. } => Some(comments),
            AssignmentOperationExpression::BitwiseXor { comments, .. } => Some(comments),
            AssignmentOperationExpression::LeftShift { comments, .. } => Some(comments),
            AssignmentOperationExpression::RightShift { comments, .. } => Some(comments),
            AssignmentOperationExpression::Coalesce { comments, .. } => Some(comments),
            AssignmentOperationExpression::Concat { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            AssignmentOperationExpression::Assignment { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Addition { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Subtraction { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Multiplication { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Division { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Modulo { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Exponentiation { left, .. } => left.initial_position(),
            AssignmentOperationExpression::BitwiseAnd { left, .. } => left.initial_position(),
            AssignmentOperationExpression::BitwiseOr { left, .. } => left.initial_position(),
            AssignmentOperationExpression::BitwiseXor { left, .. } => left.initial_position(),
            AssignmentOperationExpression::LeftShift { left, .. } => left.initial_position(),
            AssignmentOperationExpression::RightShift { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Coalesce { left, .. } => left.initial_position(),
            AssignmentOperationExpression::Concat { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            AssignmentOperationExpression::Assignment { right, .. } => right.final_position(),
            AssignmentOperationExpression::Addition { right, .. } => right.final_position(),
            AssignmentOperationExpression::Subtraction { right, .. } => right.final_position(),
            AssignmentOperationExpression::Multiplication { right, .. } => right.final_position(),
            AssignmentOperationExpression::Division { right, .. } => right.final_position(),
            AssignmentOperationExpression::Modulo { right, .. } => right.final_position(),
            AssignmentOperationExpression::Exponentiation { right, .. } => right.final_position(),
            AssignmentOperationExpression::BitwiseAnd { right, .. } => right.final_position(),
            AssignmentOperationExpression::BitwiseOr { right, .. } => right.final_position(),
            AssignmentOperationExpression::BitwiseXor { right, .. } => right.final_position(),
            AssignmentOperationExpression::LeftShift { right, .. } => right.final_position(),
            AssignmentOperationExpression::RightShift { right, .. } => right.final_position(),
            AssignmentOperationExpression::Coalesce { right, .. } => right.final_position(),
            AssignmentOperationExpression::Concat { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            AssignmentOperationExpression::Assignment { left, right, .. }
            | AssignmentOperationExpression::Addition { left, right, .. }
            | AssignmentOperationExpression::Subtraction { left, right, .. }
            | AssignmentOperationExpression::Multiplication { left, right, .. }
            | AssignmentOperationExpression::Division { left, right, .. }
            | AssignmentOperationExpression::Modulo { left, right, .. }
            | AssignmentOperationExpression::Exponentiation { left, right, .. }
            | AssignmentOperationExpression::BitwiseAnd { left, right, .. }
            | AssignmentOperationExpression::BitwiseOr { left, right, .. }
            | AssignmentOperationExpression::BitwiseXor { left, right, .. }
            | AssignmentOperationExpression::LeftShift { left, right, .. }
            | AssignmentOperationExpression::RightShift { left, right, .. }
            | AssignmentOperationExpression::Coalesce { left, right, .. }
            | AssignmentOperationExpression::Concat { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }
}

impl Node for BitwiseOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            BitwiseOperationExpression::And { comments, .. } => Some(comments),
            BitwiseOperationExpression::Or { comments, .. } => Some(comments),
            BitwiseOperationExpression::Xor { comments, .. } => Some(comments),
            BitwiseOperationExpression::LeftShift { comments, .. } => Some(comments),
            BitwiseOperationExpression::RightShift { comments, .. } => Some(comments),
            BitwiseOperationExpression::Not { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            BitwiseOperationExpression::And { left, .. } => left.initial_position(),
            BitwiseOperationExpression::Or { left, .. } => left.initial_position(),
            BitwiseOperationExpression::Xor { left, .. } => left.initial_position(),
            BitwiseOperationExpression::LeftShift { left, .. } => left.initial_position(),
            BitwiseOperationExpression::RightShift { left, .. } => left.initial_position(),
            BitwiseOperationExpression::Not { not, .. } => not.position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            BitwiseOperationExpression::And { right, .. } => right.final_position(),
            BitwiseOperationExpression::Or { right, .. } => right.final_position(),
            BitwiseOperationExpression::Xor { right, .. } => right.final_position(),
            BitwiseOperationExpression::LeftShift { right, .. } => right.final_position(),
            BitwiseOperationExpression::RightShift { right, .. } => right.final_position(),
            BitwiseOperationExpression::Not { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            BitwiseOperationExpression::And { left, right, .. }
            | BitwiseOperationExpression::Or { left, right, .. }
            | BitwiseOperationExpression::Xor { left, right, .. }
            | BitwiseOperationExpression::LeftShift { left, right, .. }
            | BitwiseOperationExpression::RightShift { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
            BitwiseOperationExpression::Not { right, .. } => vec![right.as_ref()],
        }
    }
}

impl Node for ComparisonOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ComparisonOperationExpression::Equal { comments, .. } => Some(comments),
            ComparisonOperationExpression::NotEqual { comments, .. } => Some(comments),
            ComparisonOperationExpression::Identical { comments, .. } => Some(comments),
            ComparisonOperationExpression::NotIdentical { comments, .. } => Some(comments),
            ComparisonOperationExpression::LessThan { comments, .. } => Some(comments),
            ComparisonOperationExpression::LessThanOrEqual { comments, .. } => Some(comments),
            ComparisonOperationExpression::GreaterThan { comments, .. } => Some(comments),
            ComparisonOperationExpression::GreaterThanOrEqual { comments, .. } => Some(comments),
            ComparisonOperationExpression::Spaceship { comments, .. } => Some(comments),
            ComparisonOperationExpression::AngledNotEqual { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ComparisonOperationExpression::Equal { left, .. } => left.initial_position(),
            ComparisonOperationExpression::NotEqual { left, .. } => left.initial_position(),
            ComparisonOperationExpression::Identical { left, .. } => left.initial_position(),
            ComparisonOperationExpression::NotIdentical { left, .. } => left.initial_position(),
            ComparisonOperationExpression::LessThan { left, .. } => left.initial_position(),
            ComparisonOperationExpression::LessThanOrEqual { left, .. } => left.initial_position(),
            ComparisonOperationExpression::GreaterThan { left, .. } => left.initial_position(),
            ComparisonOperationExpression::GreaterThanOrEqual { left, .. } => {
                left.initial_position()
            }
            ComparisonOperationExpression::Spaceship { left, .. } => left.initial_position(),
            ComparisonOperationExpression::AngledNotEqual { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ComparisonOperationExpression::Equal { right, .. } => right.final_position(),
            ComparisonOperationExpression::NotEqual { right, .. } => right.final_position(),
            ComparisonOperationExpression::Identical { right, .. } => right.final_position(),
            ComparisonOperationExpression::NotIdentical { right, .. } => right.final_position(),
            ComparisonOperationExpression::LessThan { right, .. } => right.final_position(),
            ComparisonOperationExpression::LessThanOrEqual { right, .. } => right.final_position(),
            ComparisonOperationExpression::GreaterThan { right, .. } => right.final_position(),
            ComparisonOperationExpression::GreaterThanOrEqual { right, .. } => {
                right.final_position()
            }
            ComparisonOperationExpression::Spaceship { right, .. } => right.final_position(),
            ComparisonOperationExpression::AngledNotEqual { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ComparisonOperationExpression::Equal { left, right, .. }
            | ComparisonOperationExpression::NotEqual { left, right, .. }
            | ComparisonOperationExpression::Identical { left, right, .. }
            | ComparisonOperationExpression::NotIdentical { left, right, .. }
            | ComparisonOperationExpression::LessThan { left, right, .. }
            | ComparisonOperationExpression::LessThanOrEqual { left, right, .. }
            | ComparisonOperationExpression::GreaterThan { left, right, .. }
            | ComparisonOperationExpression::GreaterThanOrEqual { left, right, .. }
            | ComparisonOperationExpression::Spaceship { left, right, .. }
            | ComparisonOperationExpression::AngledNotEqual { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }
}

impl Node for LogicalOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            LogicalOperationExpression::And { comments, .. } => Some(comments),
            LogicalOperationExpression::Or { comments, .. } => Some(comments),
            LogicalOperationExpression::Not { comments, .. } => Some(comments),
            LogicalOperationExpression::LogicalAnd { comments, .. } => Some(comments),
            LogicalOperationExpression::LogicalOr { comments, .. } => Some(comments),
            LogicalOperationExpression::LogicalXor { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            LogicalOperationExpression::And { left, .. } => left.initial_position(),
            LogicalOperationExpression::Or { left, .. } => left.initial_position(),
            LogicalOperationExpression::Not { bang, .. } => bang.position,
            LogicalOperationExpression::LogicalAnd { left, .. } => left.initial_position(),
            LogicalOperationExpression::LogicalOr { left, .. } => left.initial_position(),
            LogicalOperationExpression::LogicalXor { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            LogicalOperationExpression::And { right, .. } => right.final_position(),
            LogicalOperationExpression::Or { right, .. } => right.final_position(),
            LogicalOperationExpression::Not { right, .. } => right.final_position(),
            LogicalOperationExpression::LogicalAnd { right, .. } => right.final_position(),
            LogicalOperationExpression::LogicalOr { right, .. } => right.final_position(),
            LogicalOperationExpression::LogicalXor { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            LogicalOperationExpression::And { left, right, .. }
            | LogicalOperationExpression::Or { left, right, .. }
            | LogicalOperationExpression::LogicalAnd { left, right, .. }
            | LogicalOperationExpression::LogicalOr { left, right, .. }
            | LogicalOperationExpression::LogicalXor { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
            LogicalOperationExpression::Not { right, .. } => vec![right.as_ref()],
        }
    }
}

impl Node for StringOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            StringOperationExpression::Concat { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            StringOperationExpression::Concat { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            StringOperationExpression::Concat { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            StringOperationExpression::Concat { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }
}

impl Node for ArrayOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ArrayOperationExpression::Access { comments, .. } => Some(comments),
            ArrayOperationExpression::Push { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ArrayOperationExpression::Access { array, .. } => array.initial_position(),
            ArrayOperationExpression::Push { array, .. } => array.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ArrayOperationExpression::Access { right_bracket, .. } => right_bracket.position + 1,
            ArrayOperationExpression::Push { right_bracket, .. } => right_bracket.position + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ArrayOperationExpression::Access { array, index, .. } => {
                vec![array.as_ref(), index.as_ref()]
            }
            ArrayOperationExpression::Push { array, .. } => {
                vec![array.as_ref()]
            }
        }
    }
}

impl Node for CoalesceOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            CoalesceOperationExpression::Coalesce { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            CoalesceOperationExpression::Coalesce { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            CoalesceOperationExpression::Coalesce { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            CoalesceOperationExpression::Coalesce { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }
}

impl Node for TernaryOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            TernaryOperationExpression::Ternary { comments, .. } => Some(comments),
            TernaryOperationExpression::ShortTernary { comments, .. } => Some(comments),
            TernaryOperationExpression::ImplicitShortTernary { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            TernaryOperationExpression::Ternary { condition, .. } => condition.initial_position(),
            TernaryOperationExpression::ShortTernary { condition, .. } => {
                condition.initial_position()
            }
            TernaryOperationExpression::ImplicitShortTernary { condition, .. } => {
                condition.initial_position()
            }
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            TernaryOperationExpression::Ternary { condition, .. } => condition.final_position(),
            TernaryOperationExpression::ShortTernary { condition, .. } => {
                condition.final_position()
            }
            TernaryOperationExpression::ImplicitShortTernary { condition, .. } => {
                condition.final_position()
            }
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            TernaryOperationExpression::Ternary {
                condition,
                if_true,
                if_false,
                ..
            } => vec![condition.as_ref(), if_true.as_ref(), if_false.as_ref()],
            TernaryOperationExpression::ShortTernary {
                condition,
                if_false,
                ..
            } => vec![condition.as_ref(), if_false.as_ref()],
            TernaryOperationExpression::ImplicitShortTernary {
                condition,
                if_false,
                ..
            } => vec![condition.as_ref(), if_false.as_ref()],
        }
    }
}

impl Node for TypeOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            TypeOperationExpression::Instanceof { comments, .. } => Some(comments),
            TypeOperationExpression::Is { comments, .. } => Some(comments),
            TypeOperationExpression::As { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            TypeOperationExpression::Instanceof { left, .. } => left.initial_position(),
            TypeOperationExpression::Is { left, .. } => left.initial_position(),
            TypeOperationExpression::As { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            TypeOperationExpression::Instanceof { right, .. } => right.final_position(),
            TypeOperationExpression::Is { right, .. } => right.final_position(),
            TypeOperationExpression::As { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            TypeOperationExpression::Instanceof { left, right, .. } => vec![left.as_ref(), right],
            TypeOperationExpression::Is { left, right, .. } => vec![left.as_ref(), right],
            TypeOperationExpression::As { left, right, .. } => vec![left.as_ref(), right],
        }
    }
}

impl Node for GeneratorOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            GeneratorOperationExpression::Yield { comments, .. } => Some(comments),
            GeneratorOperationExpression::YieldValue { comments, .. } => Some(comments),
            GeneratorOperationExpression::YieldKeyValue { comments, .. } => Some(comments),
            GeneratorOperationExpression::YieldFrom { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            GeneratorOperationExpression::Yield { r#yield, .. } => r#yield.position,
            GeneratorOperationExpression::YieldValue { r#yield, .. } => r#yield.position,
            GeneratorOperationExpression::YieldKeyValue { r#yield, .. } => r#yield.position,
            GeneratorOperationExpression::YieldFrom { r#yield, .. } => r#yield.position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            GeneratorOperationExpression::Yield { r#yield, .. } => r#yield.position + 5,
            GeneratorOperationExpression::YieldValue { value, .. } => value.final_position(),
            GeneratorOperationExpression::YieldKeyValue { value, .. } => value.final_position(),
            GeneratorOperationExpression::YieldFrom { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            GeneratorOperationExpression::Yield { .. } => vec![],
            GeneratorOperationExpression::YieldValue { value, .. } => vec![value.as_ref()],
            GeneratorOperationExpression::YieldKeyValue { key, value, .. } => {
                vec![key.as_ref(), value.as_ref()]
            }
            GeneratorOperationExpression::YieldFrom { value, .. } => vec![value.as_ref()],
        }
    }
}

impl Node for ExceptionOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ExceptionOperationExpression::Throw { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ExceptionOperationExpression::Throw { r#throw, .. } => r#throw.position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ExceptionOperationExpression::Throw { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ExceptionOperationExpression::Throw { value, .. } => vec![value.as_ref()],
        }
    }
}

impl Node for ObjectOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ObjectOperationExpression::Clone { comments, .. } => Some(comments),
            ObjectOperationExpression::MethodCall { comments, .. } => Some(comments),
            ObjectOperationExpression::NullsafeMethodCall { comments, .. } => Some(comments),
            ObjectOperationExpression::MethodClosureCreation { comments, .. } => Some(comments),
            ObjectOperationExpression::PropertyFetch { comments, .. } => Some(comments),
            ObjectOperationExpression::NullsafePropertyFetch { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ObjectOperationExpression::Clone { clone, .. } => clone.position,
            ObjectOperationExpression::MethodCall { object, .. } => object.initial_position(),
            ObjectOperationExpression::NullsafeMethodCall { object, .. } => {
                object.initial_position()
            }
            ObjectOperationExpression::MethodClosureCreation { object, .. } => {
                object.initial_position()
            }
            ObjectOperationExpression::PropertyFetch { object, .. } => object.initial_position(),
            ObjectOperationExpression::NullsafePropertyFetch { object, .. } => {
                object.initial_position()
            }
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ObjectOperationExpression::Clone { object, .. } => object.final_position(),
            ObjectOperationExpression::MethodCall { arguments, .. } => arguments.final_position(),
            ObjectOperationExpression::NullsafeMethodCall { arguments, .. } => {
                arguments.final_position()
            }
            ObjectOperationExpression::MethodClosureCreation { placeholder, .. } => {
                placeholder.final_position()
            }
            ObjectOperationExpression::PropertyFetch { property, .. } => property.final_position(),
            ObjectOperationExpression::NullsafePropertyFetch { property, .. } => {
                property.final_position()
            }
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ObjectOperationExpression::Clone { object, .. } => vec![object.as_ref()],
            ObjectOperationExpression::MethodCall {
                object,
                method,
                generics,
                arguments,
                ..
            }
            | ObjectOperationExpression::NullsafeMethodCall {
                object,
                method,
                generics,
                arguments,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![object.as_ref(), method];
                if let Some(generics) = generics {
                    children.push(generics);
                }

                children.push(arguments);

                children
            }
            ObjectOperationExpression::MethodClosureCreation {
                object,
                method,
                generics,
                placeholder,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![object.as_ref(), method];
                if let Some(generics) = generics {
                    children.push(generics);
                }

                children.push(placeholder);

                children
            }
            ObjectOperationExpression::PropertyFetch {
                object, property, ..
            }
            | ObjectOperationExpression::NullsafePropertyFetch {
                object, property, ..
            } => {
                vec![object.as_ref(), property]
            }
        }
    }
}

impl Node for ClassOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ClassOperationExpression::Initialization { comments, .. } => Some(comments),
            ClassOperationExpression::AnonymousInitialization { comments, .. } => Some(comments),
            ClassOperationExpression::StaticMethodCall { comments, .. } => Some(comments),
            ClassOperationExpression::StaticMethodClosureCreation { comments, .. } => {
                Some(comments)
            }
            ClassOperationExpression::StaticPropertyFetch { comments, .. } => Some(comments),
            ClassOperationExpression::ConstantFetch { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ClassOperationExpression::Initialization { new, .. } => new.position,
            ClassOperationExpression::AnonymousInitialization { new, .. } => new.position,
            ClassOperationExpression::StaticMethodCall { class, .. } => class.initial_position(),
            ClassOperationExpression::StaticMethodClosureCreation { class, .. } => {
                class.initial_position()
            }
            ClassOperationExpression::StaticPropertyFetch { class, .. } => class.initial_position(),
            ClassOperationExpression::ConstantFetch { class, .. } => class.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ClassOperationExpression::Initialization { arguments, .. } => {
                arguments.final_position()
            }
            ClassOperationExpression::AnonymousInitialization { class, .. } => {
                class.final_position()
            }
            ClassOperationExpression::StaticMethodCall { arguments, .. } => {
                arguments.final_position()
            }
            ClassOperationExpression::StaticMethodClosureCreation { placeholder, .. } => {
                placeholder.final_position()
            }
            ClassOperationExpression::StaticPropertyFetch { property, .. } => {
                property.final_position()
            }
            ClassOperationExpression::ConstantFetch { constant, .. } => constant.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ClassOperationExpression::Initialization {
                class,
                generics,
                arguments,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![class];
                if let Some(generics) = generics {
                    children.push(generics);
                }

                children.push(arguments);

                children
            }
            ClassOperationExpression::AnonymousInitialization { class, .. } => {
                vec![class]
            }
            ClassOperationExpression::StaticMethodCall {
                class,
                method,
                generics,
                arguments,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![class.as_ref(), method];
                if let Some(generics) = generics {
                    children.push(generics);
                }

                children.push(arguments);

                children
            }
            ClassOperationExpression::StaticMethodClosureCreation {
                class,
                generics,
                method,
                placeholder,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![class.as_ref(), method];
                if let Some(generics) = generics {
                    children.push(generics);
                }

                children.push(placeholder);

                children
            }
            ClassOperationExpression::StaticPropertyFetch {
                class, property, ..
            } => {
                vec![class.as_ref(), property]
            }
            ClassOperationExpression::ConstantFetch {
                class, constant, ..
            } => {
                vec![class.as_ref(), constant]
            }
        }
    }
}

impl Node for FunctionOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            FunctionOperationExpression::Call { comments, .. } => Some(comments),
            FunctionOperationExpression::ClosureCreation { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            FunctionOperationExpression::Call { function, .. } => function.initial_position(),
            FunctionOperationExpression::ClosureCreation { function, .. } => {
                function.initial_position()
            }
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            FunctionOperationExpression::Call { arguments, .. } => arguments.final_position(),
            FunctionOperationExpression::ClosureCreation { placeholder, .. } => {
                placeholder.final_position()
            }
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            FunctionOperationExpression::Call {
                function,
                generics,
                arguments,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![];
                children.push(function.as_ref());
                if let Some(generics) = generics {
                    children.push(generics);
                }
                children.push(arguments);

                children
            }
            FunctionOperationExpression::ClosureCreation {
                function,
                generics,
                placeholder,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![];
                children.push(function.as_ref());
                if let Some(generics) = generics {
                    children.push(generics);
                }
                children.push(placeholder);

                children
            }
        }
    }
}

impl Node for AsyncOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            AsyncOperationExpression::Await { comments, .. } => Some(comments),
            AsyncOperationExpression::Async { comments, .. } => Some(comments),
            AsyncOperationExpression::Concurrently { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            AsyncOperationExpression::Await { r#await, .. } => r#await.position,
            AsyncOperationExpression::Async { r#async, .. } => r#async.position,
            AsyncOperationExpression::Concurrently { concurrently, .. } => concurrently.position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            AsyncOperationExpression::Await { expression, .. } => expression.final_position(),
            AsyncOperationExpression::Async { expression, .. } => expression.final_position(),
            AsyncOperationExpression::Concurrently { right_brace, .. } => right_brace.position + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            AsyncOperationExpression::Await { expression, .. } => vec![expression.as_ref()],
            AsyncOperationExpression::Async { expression, .. } => vec![expression.as_ref()],
            AsyncOperationExpression::Concurrently { expressions, .. } => {
                expressions.inner.iter().map(|e| e as &dyn Node).collect()
            }
        }
    }
}
