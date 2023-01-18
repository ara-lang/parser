use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::expression::argument::ArgumentListExpression;
use crate::tree::expression::argument::ArgumentPlaceholderExpression;
use crate::tree::expression::class::AnonymousClassExpression;
use crate::tree::expression::generic::GenericGroupExpression;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ArithmeticOperationExpression {
    Addition {
        comments: CommentGroup,
        left: Box<Expression>,
        plus: usize,
        right: Box<Expression>,
    },
    Subtraction {
        comments: CommentGroup,
        left: Box<Expression>,
        minus: usize,
        right: Box<Expression>,
    },
    Multiplication {
        comments: CommentGroup,
        left: Box<Expression>,
        asterisk: usize,
        right: Box<Expression>,
    },
    Division {
        comments: CommentGroup,
        left: Box<Expression>,
        slash: usize,
        right: Box<Expression>,
    },
    Modulo {
        comments: CommentGroup,
        left: Box<Expression>,
        percent: usize,
        right: Box<Expression>,
    },
    Exponentiation {
        comments: CommentGroup,
        left: Box<Expression>,
        pow: usize,
        right: Box<Expression>,
    },
    Negative {
        comments: CommentGroup,
        minus: usize,
        right: Box<Expression>,
    },
    Positive {
        comments: CommentGroup,
        plus: usize,
        right: Box<Expression>,
    },
    PreIncrement {
        comments: CommentGroup,
        increment: usize,
        right: Box<Expression>,
    },
    PostIncrement {
        left: Box<Expression>,
        increment: usize,
    },
    PreDecrement {
        comments: CommentGroup,
        decrement: usize,
        right: Box<Expression>,
    },
    PostDecrement {
        left: Box<Expression>,
        decrement: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum AssignmentOperationExpression {
    Assignment {
        comments: CommentGroup,
        left: Box<Expression>,
        equals: usize,
        right: Box<Expression>,
    },
    Addition {
        comments: CommentGroup,
        left: Box<Expression>,
        plus_equals: usize,
        right: Box<Expression>,
    },
    Subtraction {
        comments: CommentGroup,
        left: Box<Expression>,
        minus_equals: usize,
        right: Box<Expression>,
    },
    Multiplication {
        comments: CommentGroup,
        left: Box<Expression>,
        asterisk_equals: usize,
        right: Box<Expression>,
    },
    Division {
        comments: CommentGroup,
        left: Box<Expression>,
        slash_equals: usize,
        right: Box<Expression>,
    },
    Modulo {
        comments: CommentGroup,
        left: Box<Expression>,
        percent_equals: usize,
        right: Box<Expression>,
    },
    Exponentiation {
        comments: CommentGroup,
        left: Box<Expression>,
        pow_equals: usize,
        right: Box<Expression>,
    },
    Concat {
        comments: CommentGroup,
        left: Box<Expression>,
        dot_equals: usize,
        right: Box<Expression>,
    },
    BitwiseAnd {
        comments: CommentGroup,
        left: Box<Expression>,
        ampersand_equals: usize,
        right: Box<Expression>,
    },
    BitwiseOr {
        comments: CommentGroup,
        left: Box<Expression>,
        pipe_equals: usize,
        right: Box<Expression>,
    },
    BitwiseXor {
        comments: CommentGroup,
        left: Box<Expression>,
        caret_equals: usize,
        right: Box<Expression>,
    },
    LeftShift {
        comments: CommentGroup,
        left: Box<Expression>,
        left_shift_equals: usize,
        right: Box<Expression>,
    },
    RightShift {
        comments: CommentGroup,
        left: Box<Expression>,
        right_shift_equals: usize,
        right: Box<Expression>,
    },
    Coalesce {
        comments: CommentGroup,
        left: Box<Expression>,
        coalesce_equals: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum BitwiseOperationExpression {
    And {
        comments: CommentGroup,
        left: Box<Expression>,
        and: usize,
        right: Box<Expression>,
    },
    Or {
        comments: CommentGroup,
        left: Box<Expression>,
        or: usize,
        right: Box<Expression>,
    },
    Xor {
        comments: CommentGroup,
        left: Box<Expression>,
        xor: usize,
        right: Box<Expression>,
    },
    LeftShift {
        comments: CommentGroup,
        left: Box<Expression>,
        left_shift: usize,
        right: Box<Expression>,
    },
    RightShift {
        comments: CommentGroup,
        left: Box<Expression>,
        right_shift: usize,
        right: Box<Expression>,
    },
    Not {
        comments: CommentGroup,
        not: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ComparisonOperationExpression {
    Equal {
        comments: CommentGroup,
        left: Box<Expression>,
        double_equals: usize,
        right: Box<Expression>,
    },
    Identical {
        comments: CommentGroup,
        left: Box<Expression>,
        triple_equals: usize,
        right: Box<Expression>,
    },
    NotEqual {
        comments: CommentGroup,
        left: Box<Expression>,
        bang_equals: usize,
        right: Box<Expression>,
    },
    NotIdentical {
        comments: CommentGroup,
        left: Box<Expression>,
        bang_double_equals: usize,
        right: Box<Expression>,
    },
    LessThan {
        comments: CommentGroup,
        left: Box<Expression>,
        less_than: usize,
        right: Box<Expression>,
    },
    GreaterThan {
        comments: CommentGroup,
        left: Box<Expression>,
        greater_than: usize,
        right: Box<Expression>,
    },
    LessThanOrEqual {
        comments: CommentGroup,
        left: Box<Expression>,
        less_than_equals: usize,
        right: Box<Expression>,
    },
    GreaterThanOrEqual {
        comments: CommentGroup,
        left: Box<Expression>,
        greater_than_equals: usize,
        right: Box<Expression>,
    },
    Spaceship {
        comments: CommentGroup,
        left: Box<Expression>,
        spaceship: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum LogicalOperationExpression {
    And {
        comments: CommentGroup,
        left: Box<Expression>,
        double_ampersand: usize,
        right: Box<Expression>,
    },
    Or {
        comments: CommentGroup,
        left: Box<Expression>,
        double_pipe: usize,
        right: Box<Expression>,
    },
    Not {
        comments: CommentGroup,
        bang: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum StringOperationExpression {
    Concat {
        comments: CommentGroup,
        left: Box<Expression>,
        dot: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ArrayOperationExpression {
    Access {
        comments: CommentGroup,
        array: Box<Expression>,
        left_bracket: usize,
        index: Box<Expression>,
        right_bracket: usize,
    },
    Push {
        comments: CommentGroup,
        array: Box<Expression>,
        left_bracket: usize,
        right_bracket: usize,
    },
    Unset {
        comments: CommentGroup,
        unset: Keyword,
        item: Box<Expression>,
    },
    Isset {
        comments: CommentGroup,
        isset: Keyword,
        item: Box<Expression>,
    },
    In {
        comments: CommentGroup,
        item: Box<Expression>,
        r#in: Keyword,
        array: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum CoalesceOperationExpression {
    Coalesce {
        comments: CommentGroup,
        left: Box<Expression>,
        double_question: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TernaryOperationExpression {
    Ternary {
        comments: CommentGroup,
        condition: Box<Expression>,
        question: usize,
        if_true: Box<Expression>,
        colon: usize,
        if_false: Box<Expression>,
    },
    ImplicitShortTernary {
        comments: CommentGroup,
        condition: Box<Expression>,
        question: usize,
        colon: usize,
        if_false: Box<Expression>,
    },
    ShortTernary {
        comments: CommentGroup,
        condition: Box<Expression>,
        question_colon: usize,
        if_false: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TypeOperationExpression {
    Instanceof {
        comments: CommentGroup,
        left: Box<Expression>,
        instanceof: Keyword,
        right: Identifier,
    },
    Is {
        comments: CommentGroup,
        left: Box<Expression>,
        is: Keyword,
        right: TypeDefinition,
    },
    Into {
        comments: CommentGroup,
        left: Box<Expression>,
        into: Keyword,
        right: TypeDefinition,
    },
    As {
        comments: CommentGroup,
        left: Box<Expression>,
        r#as: Keyword,
        right: TypeDefinition,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum GeneratorOperationExpression {
    Yield {
        comments: CommentGroup,
        r#yield: Keyword,
    },
    YieldValue {
        comments: CommentGroup,
        r#yield: Keyword,
        value: Box<Expression>,
    },
    YieldKeyValue {
        comments: CommentGroup,
        r#yield: Keyword,
        key: Box<Expression>,
        double_arrow: usize,
        value: Box<Expression>,
    },
    YieldFrom {
        comments: CommentGroup,
        r#yield: Keyword,
        from: Keyword,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ExceptionOperationExpression {
    Throw {
        comments: CommentGroup,
        r#throw: Keyword,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ObjectOperationExpression {
    Clone {
        comments: CommentGroup,
        clone: Keyword,
        object: Box<Expression>,
    },
    MethodCall {
        comments: CommentGroup,
        object: Box<Expression>,
        arrow: usize,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    NullsafeMethodCall {
        comments: CommentGroup,
        object: Box<Expression>,
        question_arrow: usize,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    MethodClosureCreation {
        comments: CommentGroup,
        object: Box<Expression>,
        arrow: usize,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        placeholder: ArgumentPlaceholderExpression,
    },
    PropertyFetch {
        comments: CommentGroup,
        object: Box<Expression>,
        arrow: usize,
        property: Identifier,
    },
    NullsafePropertyFetch {
        comments: CommentGroup,
        object: Box<Expression>,
        question_arrow: usize,
        property: Identifier,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ClassOperationInitializationClassExpression {
    Identifier(Identifier),
    Variable(Variable),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ClassOperationExpression {
    Initialization {
        comments: CommentGroup,
        new: Keyword,
        class: ClassOperationInitializationClassExpression,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    AnonymousInitialization {
        comments: CommentGroup,
        new: Keyword,
        class: AnonymousClassExpression,
    },
    StaticMethodCall {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: usize,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        arguments: ArgumentListExpression,
    },
    StaticMethodClosureCreation {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: usize,
        method: Identifier,
        generics: Option<GenericGroupExpression>,
        placeholder: ArgumentPlaceholderExpression,
    },
    StaticPropertyFetch {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: usize,
        property: Variable,
    },
    ConstantFetch {
        comments: CommentGroup,
        class: Box<Expression>,
        double_colon: usize,
        constant: Identifier,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum AsyncOperationExpression {
    Async {
        comments: CommentGroup,
        r#async: Keyword,
        expression: Box<Expression>,
    },
    Await {
        comments: CommentGroup,
        r#await: Keyword,
        expression: Box<Expression>,
    },
    Concurrently {
        comments: CommentGroup,
        concurrently: Keyword,
        left_brace: usize,
        expressions: CommaSeparated<Expression>,
        right_brace: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum RangeOperationExpression {
    Between {
        comments: CommentGroup,
        from: Box<Expression>,
        double_dot: usize,
        to: Box<Expression>,
    },
    BetweenInclusive {
        comments: CommentGroup,
        from: Box<Expression>,
        double_dot: usize,
        equals: usize,
        to: Box<Expression>,
    },
    To {
        comments: CommentGroup,
        double_dot: usize,
        to: Box<Expression>,
    },
    ToInclusive {
        comments: CommentGroup,
        double_dot: usize,
        equals: usize,
        to: Box<Expression>,
    },
    From {
        comments: CommentGroup,
        from: Box<Expression>,
        double_dot: usize,
    },
    Full {
        comments: CommentGroup,
        double_dot: usize,
    },
}

impl RangeOperationExpression {
    pub fn has_start(&self) -> bool {
        match self {
            RangeOperationExpression::Between { .. } => true,
            RangeOperationExpression::BetweenInclusive { .. } => true,
            RangeOperationExpression::To { .. } => false,
            RangeOperationExpression::ToInclusive { .. } => false,
            RangeOperationExpression::From { .. } => true,
            RangeOperationExpression::Full { .. } => false,
        }
    }
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
            ArithmeticOperationExpression::Negative { minus, .. } => *minus,
            ArithmeticOperationExpression::Positive { plus, .. } => *plus,
            ArithmeticOperationExpression::PreIncrement { increment, .. } => *increment,
            ArithmeticOperationExpression::PreDecrement { decrement, .. } => *decrement,
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
            ArithmeticOperationExpression::PostIncrement { increment, .. } => *increment,
            ArithmeticOperationExpression::PostDecrement { decrement, .. } => *decrement,
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

    fn get_description(&self) -> String {
        "arithmetic operation expression".to_string()
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

    fn get_description(&self) -> String {
        "assignment operation expression".to_string()
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
            BitwiseOperationExpression::Not { not, .. } => *not,
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

    fn get_description(&self) -> String {
        "bitwise operation expression".to_string()
    }
}

impl Node for ComparisonOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ComparisonOperationExpression::Equal { comments, .. }
            | ComparisonOperationExpression::NotEqual { comments, .. }
            | ComparisonOperationExpression::Identical { comments, .. }
            | ComparisonOperationExpression::NotIdentical { comments, .. }
            | ComparisonOperationExpression::LessThan { comments, .. }
            | ComparisonOperationExpression::LessThanOrEqual { comments, .. }
            | ComparisonOperationExpression::GreaterThan { comments, .. }
            | ComparisonOperationExpression::GreaterThanOrEqual { comments, .. }
            | ComparisonOperationExpression::Spaceship { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ComparisonOperationExpression::Equal { left, .. }
            | ComparisonOperationExpression::NotEqual { left, .. }
            | ComparisonOperationExpression::Identical { left, .. }
            | ComparisonOperationExpression::NotIdentical { left, .. }
            | ComparisonOperationExpression::LessThan { left, .. }
            | ComparisonOperationExpression::LessThanOrEqual { left, .. }
            | ComparisonOperationExpression::GreaterThan { left, .. }
            | ComparisonOperationExpression::GreaterThanOrEqual { left, .. }
            | ComparisonOperationExpression::Spaceship { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ComparisonOperationExpression::Equal { right, .. }
            | ComparisonOperationExpression::NotEqual { right, .. }
            | ComparisonOperationExpression::Identical { right, .. }
            | ComparisonOperationExpression::NotIdentical { right, .. }
            | ComparisonOperationExpression::LessThan { right, .. }
            | ComparisonOperationExpression::LessThanOrEqual { right, .. }
            | ComparisonOperationExpression::GreaterThan { right, .. }
            | ComparisonOperationExpression::GreaterThanOrEqual { right, .. }
            | ComparisonOperationExpression::Spaceship { right, .. } => right.final_position(),
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
            | ComparisonOperationExpression::Spaceship { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        "comparison operation expression".to_string()
    }
}

impl Node for LogicalOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            LogicalOperationExpression::And { comments, .. } => Some(comments),
            LogicalOperationExpression::Or { comments, .. } => Some(comments),
            LogicalOperationExpression::Not { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            LogicalOperationExpression::And { left, .. } => left.initial_position(),
            LogicalOperationExpression::Or { left, .. } => left.initial_position(),
            LogicalOperationExpression::Not { bang, .. } => *bang,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            LogicalOperationExpression::And { right, .. } => right.final_position(),
            LogicalOperationExpression::Or { right, .. } => right.final_position(),
            LogicalOperationExpression::Not { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            LogicalOperationExpression::And { left, right, .. }
            | LogicalOperationExpression::Or { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
            LogicalOperationExpression::Not { right, .. } => vec![right.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        "logical operation expression".to_string()
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

    fn get_description(&self) -> String {
        "string operation expression".to_string()
    }
}

impl Node for ArrayOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ArrayOperationExpression::Access { comments, .. }
            | ArrayOperationExpression::Push { comments, .. }
            | ArrayOperationExpression::Isset { comments, .. }
            | ArrayOperationExpression::Unset { comments, .. }
            | ArrayOperationExpression::In { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ArrayOperationExpression::Access { array, .. }
            | ArrayOperationExpression::Push { array, .. } => array.initial_position(),
            ArrayOperationExpression::Isset { isset, .. } => isset.initial_position(),
            ArrayOperationExpression::Unset { unset, .. } => unset.initial_position(),
            ArrayOperationExpression::In { item, .. } => item.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ArrayOperationExpression::Access { right_bracket, .. }
            | ArrayOperationExpression::Push { right_bracket, .. } => right_bracket + 1,
            ArrayOperationExpression::Isset { item, .. }
            | ArrayOperationExpression::Unset { item, .. } => item.final_position(),
            ArrayOperationExpression::In { array, .. } => array.final_position(),
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
            ArrayOperationExpression::Isset {
                isset: keyword,
                item,
                ..
            }
            | ArrayOperationExpression::Unset {
                unset: keyword,
                item,
                ..
            } => {
                vec![keyword, item.as_ref()]
            }
            ArrayOperationExpression::In {
                item, r#in, array, ..
            } => {
                vec![item.as_ref(), r#in, array.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        "array operation expression".to_string()
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

    fn get_description(&self) -> String {
        "coalesce operation expression".to_string()
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
            TernaryOperationExpression::Ternary { if_false, .. } => if_false.final_position(),
            TernaryOperationExpression::ShortTernary { if_false, .. } => if_false.final_position(),
            TernaryOperationExpression::ImplicitShortTernary { if_false, .. } => {
                if_false.final_position()
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

    fn get_description(&self) -> String {
        "ternary operation expression".to_string()
    }
}

impl Node for TypeOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            TypeOperationExpression::Instanceof { comments, .. } => Some(comments),
            TypeOperationExpression::Is { comments, .. } => Some(comments),
            TypeOperationExpression::Into { comments, .. } => Some(comments),
            TypeOperationExpression::As { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            TypeOperationExpression::Instanceof { left, .. } => left.initial_position(),
            TypeOperationExpression::Is { left, .. } => left.initial_position(),
            TypeOperationExpression::Into { left, .. } => left.initial_position(),
            TypeOperationExpression::As { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            TypeOperationExpression::Instanceof { right, .. } => right.final_position(),
            TypeOperationExpression::Is { right, .. } => right.final_position(),
            TypeOperationExpression::Into { right, .. } => right.final_position(),
            TypeOperationExpression::As { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            TypeOperationExpression::Instanceof {
                left,
                instanceof,
                right,
                ..
            } => vec![left.as_ref(), instanceof, right],
            TypeOperationExpression::Is {
                left, is, right, ..
            } => vec![left.as_ref(), is, right],
            TypeOperationExpression::Into {
                left, into, right, ..
            } => vec![left.as_ref(), into, right],
            TypeOperationExpression::As {
                left, r#as, right, ..
            } => vec![left.as_ref(), r#as, right],
        }
    }

    fn get_description(&self) -> String {
        "type operation expression".to_string()
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
            GeneratorOperationExpression::Yield { r#yield, .. }
            | GeneratorOperationExpression::YieldValue { r#yield, .. }
            | GeneratorOperationExpression::YieldKeyValue { r#yield, .. }
            | GeneratorOperationExpression::YieldFrom { r#yield, .. } => r#yield.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            GeneratorOperationExpression::Yield { r#yield, .. } => r#yield.final_position(),
            GeneratorOperationExpression::YieldValue { value, .. } => value.final_position(),
            GeneratorOperationExpression::YieldKeyValue { value, .. } => value.final_position(),
            GeneratorOperationExpression::YieldFrom { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            GeneratorOperationExpression::Yield { r#yield, .. } => vec![r#yield],
            GeneratorOperationExpression::YieldValue { r#yield, value, .. } => {
                vec![r#yield, value.as_ref()]
            }
            GeneratorOperationExpression::YieldKeyValue {
                r#yield,
                key,
                value,
                ..
            } => {
                vec![r#yield, key.as_ref(), value.as_ref()]
            }
            GeneratorOperationExpression::YieldFrom {
                r#yield,
                from,
                value,
                ..
            } => vec![r#yield, from, value.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        "generator operation expression".to_string()
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
            ExceptionOperationExpression::Throw { throw, .. } => throw.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ExceptionOperationExpression::Throw { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ExceptionOperationExpression::Throw { throw, value, .. } => vec![throw, value.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        "exception operation expression".to_string()
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
            ObjectOperationExpression::Clone { clone, .. } => clone.initial_position(),
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
            ObjectOperationExpression::Clone { clone, object, .. } => vec![clone, object.as_ref()],
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

    fn get_description(&self) -> String {
        "object operation expression".to_string()
    }
}

impl Node for ClassOperationInitializationClassExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ClassOperationInitializationClassExpression::Identifier(_) => None,
            ClassOperationInitializationClassExpression::Variable(_) => None,
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ClassOperationInitializationClassExpression::Identifier(identifier) => {
                identifier.initial_position()
            }
            ClassOperationInitializationClassExpression::Variable(variable) => {
                variable.initial_position()
            }
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ClassOperationInitializationClassExpression::Identifier(identifier) => {
                identifier.final_position()
            }
            ClassOperationInitializationClassExpression::Variable(variable) => {
                variable.final_position()
            }
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ClassOperationInitializationClassExpression::Identifier(identifier) => vec![identifier],
            ClassOperationInitializationClassExpression::Variable(variable) => vec![variable],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            ClassOperationInitializationClassExpression::Identifier(identifier) => {
                identifier.get_description()
            }
            ClassOperationInitializationClassExpression::Variable(variable) => {
                variable.get_description()
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
            ClassOperationExpression::Initialization { new, .. }
            | ClassOperationExpression::AnonymousInitialization { new, .. } => {
                new.initial_position()
            }
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
                new,
                class,
                generics,
                arguments,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![new, class];
                if let Some(generics) = generics {
                    children.push(generics);
                }

                children.push(arguments);

                children
            }
            ClassOperationExpression::AnonymousInitialization { new, class, .. } => {
                vec![new, class]
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

    fn get_description(&self) -> String {
        "class operation expression".to_string()
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

    fn get_description(&self) -> String {
        "function operation expression".to_string()
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
            AsyncOperationExpression::Await { r#await, .. } => r#await.initial_position(),
            AsyncOperationExpression::Async { r#async, .. } => r#async.initial_position(),
            AsyncOperationExpression::Concurrently { concurrently, .. } => {
                concurrently.initial_position()
            }
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            AsyncOperationExpression::Await { expression, .. } => expression.final_position(),
            AsyncOperationExpression::Async { expression, .. } => expression.final_position(),
            AsyncOperationExpression::Concurrently { right_brace, .. } => right_brace + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            AsyncOperationExpression::Await {
                r#await,
                expression,
                ..
            } => vec![r#await, expression.as_ref()],
            AsyncOperationExpression::Async {
                r#async,
                expression,
                ..
            } => vec![r#async, expression.as_ref()],
            AsyncOperationExpression::Concurrently {
                concurrently,
                expressions,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![concurrently];
                for expression in &expressions.inner {
                    children.push(expression);
                }

                children
            }
        }
    }

    fn get_description(&self) -> String {
        "async operation expression".to_string()
    }
}

impl Node for RangeOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            RangeOperationExpression::Between { comments, .. } => Some(comments),
            RangeOperationExpression::BetweenInclusive { comments, .. } => Some(comments),
            RangeOperationExpression::To { comments, .. } => Some(comments),
            RangeOperationExpression::ToInclusive { comments, .. } => Some(comments),
            RangeOperationExpression::From { comments, .. } => Some(comments),
            RangeOperationExpression::Full { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            RangeOperationExpression::Between { from, .. } => from.initial_position(),
            RangeOperationExpression::BetweenInclusive { from, .. } => from.initial_position(),
            RangeOperationExpression::To { double_dot, .. } => *double_dot,
            RangeOperationExpression::ToInclusive { double_dot, .. } => *double_dot,
            RangeOperationExpression::From { from, .. } => from.initial_position(),
            RangeOperationExpression::Full { double_dot, .. } => *double_dot,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            RangeOperationExpression::Between { to, .. } => to.final_position(),
            RangeOperationExpression::BetweenInclusive { to, .. } => to.final_position(),
            RangeOperationExpression::To { to, .. } => to.final_position(),
            RangeOperationExpression::ToInclusive { to, .. } => to.final_position(),
            RangeOperationExpression::From { double_dot, .. } => *double_dot,
            RangeOperationExpression::Full { double_dot, .. } => *double_dot,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            RangeOperationExpression::Between { from, to, .. } => {
                vec![from.as_ref(), to.as_ref()]
            }
            RangeOperationExpression::BetweenInclusive { from, to, .. } => {
                vec![from.as_ref(), to.as_ref()]
            }
            RangeOperationExpression::To { to, .. } => vec![to.as_ref()],
            RangeOperationExpression::ToInclusive { to, .. } => vec![to.as_ref()],
            RangeOperationExpression::From { from, .. } => vec![from.as_ref()],
            RangeOperationExpression::Full { .. } => vec![],
        }
    }

    fn get_description(&self) -> String {
        "range operation expression".to_string()
    }
}
