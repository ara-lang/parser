use bincode::Decode;
use bincode::Encode;
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum FunctionalOperationExpression {
    Pipe {
        comments: CommentGroup,
        left: Box<Expression>,
        pipe: usize,
        greater_than: usize,
        right: Box<Expression>,
    },
    Expression {
        comments: CommentGroup,
        dollar: usize,
        generics: Option<GenericGroupExpression>,
        left_parenthesis: usize,
        expression: Box<Expression>,
        right_parenthesis: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum StringOperationExpression {
    Concat {
        comments: CommentGroup,
        left: Box<Expression>,
        dot: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum CoalesceOperationExpression {
    Coalesce {
        comments: CommentGroup,
        left: Box<Expression>,
        double_question: usize,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ExceptionOperationExpression {
    Throw {
        comments: CommentGroup,
        r#throw: Keyword,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ClassOperationInitializationClassExpression {
    Identifier(Identifier),
    Variable(Variable),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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
        match &self {
            Self::Between { .. } => true,
            Self::BetweenInclusive { .. } => true,
            Self::To { .. } => false,
            Self::ToInclusive { .. } => false,
            Self::From { .. } => true,
            Self::Full { .. } => false,
        }
    }
}

impl Node for FunctionalOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match self {
            Self::Pipe { comments, .. } | Self::Expression { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Pipe { left, .. } => left.initial_position(),
            Self::Expression { dollar, .. } => *dollar,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Pipe { right, .. } => right.final_position(),
            Self::Expression {
                right_parenthesis, ..
            } => *right_parenthesis,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Pipe { left, right, .. } => vec![left.as_ref(), right.as_ref()],
            Self::Expression {
                generics,
                expression,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![expression.as_ref()];
                if let Some(generics) = generics {
                    children.push(generics);
                }
                children
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Pipe { .. } => "pipe functional operation expression".to_string(),
            Self::Expression { .. } => "functional operation expression".to_string(),
        }
    }
}

impl Node for ArithmeticOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Addition { comments, .. } => Some(comments),
            Self::Subtraction { comments, .. } => Some(comments),
            Self::Multiplication { comments, .. } => Some(comments),
            Self::Division { comments, .. } => Some(comments),
            Self::Modulo { comments, .. } => Some(comments),
            Self::Exponentiation { comments, .. } => Some(comments),
            Self::Negative { comments, .. } => Some(comments),
            Self::Positive { comments, .. } => Some(comments),
            Self::PreIncrement { comments, .. } => Some(comments),
            Self::PreDecrement { comments, .. } => Some(comments),
            Self::PostIncrement { .. } => None,
            Self::PostDecrement { .. } => None,
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Addition { left, .. } => left.initial_position(),
            Self::Subtraction { left, .. } => left.initial_position(),
            Self::Multiplication { left, .. } => left.initial_position(),
            Self::Division { left, .. } => left.initial_position(),
            Self::Modulo { left, .. } => left.initial_position(),
            Self::Exponentiation { left, .. } => left.initial_position(),
            Self::Negative { minus, .. } => *minus,
            Self::Positive { plus, .. } => *plus,
            Self::PreIncrement { increment, .. } => *increment,
            Self::PreDecrement { decrement, .. } => *decrement,
            Self::PostIncrement { left, .. } => left.initial_position(),
            Self::PostDecrement { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Addition { right, .. } => right.final_position(),
            Self::Subtraction { right, .. } => right.final_position(),
            Self::Multiplication { right, .. } => right.final_position(),
            Self::Division { right, .. } => right.final_position(),
            Self::Modulo { right, .. } => right.final_position(),
            Self::Exponentiation { right, .. } => right.final_position(),
            Self::Negative { right, .. } => right.final_position(),
            Self::Positive { right, .. } => right.final_position(),
            Self::PreIncrement { right, .. } => right.final_position(),
            Self::PreDecrement { right, .. } => right.final_position(),
            Self::PostIncrement { increment, .. } => *increment,
            Self::PostDecrement { decrement, .. } => *decrement,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Addition { left, right, .. }
            | Self::Subtraction { left, right, .. }
            | Self::Multiplication { left, right, .. }
            | Self::Division { left, right, .. }
            | Self::Modulo { left, right, .. }
            | Self::Exponentiation { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
            Self::Negative { right, .. }
            | Self::Positive { right, .. }
            | Self::PreIncrement { right, .. }
            | Self::PreDecrement { right, .. } => vec![right.as_ref()],
            Self::PostIncrement { left, .. } | Self::PostDecrement { left, .. } => {
                vec![left.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Addition { .. } => "addition arithmetic operation expression".to_string(),
            Self::Subtraction { .. } => "subtraction arithmetic operation expression".to_string(),
            Self::Multiplication { .. } => {
                "multiplication arithmetic operation expression".to_string()
            }
            Self::Division { .. } => "division arithmetic operation expression".to_string(),
            Self::Modulo { .. } => "modulo arithmetic operation expression".to_string(),
            Self::Exponentiation { .. } => {
                "exponentiation arithmetic operation expression".to_string()
            }
            Self::Negative { .. } => "negative arithmetic operation expression".to_string(),
            Self::Positive { .. } => "positive arithmetic operation expression".to_string(),
            Self::PreIncrement { .. } => {
                "pre-increment arithmetic operation expression".to_string()
            }
            Self::PreDecrement { .. } => {
                "pre-decrement arithmetic operation expression".to_string()
            }
            Self::PostIncrement { .. } => {
                "post-increment arithmetic operation expression".to_string()
            }
            Self::PostDecrement { .. } => {
                "post-decrement arithmetic operation expression".to_string()
            }
        }
    }
}

impl Node for AssignmentOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Assignment { comments, .. } => Some(comments),
            Self::Addition { comments, .. } => Some(comments),
            Self::Subtraction { comments, .. } => Some(comments),
            Self::Multiplication { comments, .. } => Some(comments),
            Self::Division { comments, .. } => Some(comments),
            Self::Modulo { comments, .. } => Some(comments),
            Self::Exponentiation { comments, .. } => Some(comments),
            Self::BitwiseAnd { comments, .. } => Some(comments),
            Self::BitwiseOr { comments, .. } => Some(comments),
            Self::BitwiseXor { comments, .. } => Some(comments),
            Self::LeftShift { comments, .. } => Some(comments),
            Self::RightShift { comments, .. } => Some(comments),
            Self::Coalesce { comments, .. } => Some(comments),
            Self::Concat { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Assignment { left, .. } => left.initial_position(),
            Self::Addition { left, .. } => left.initial_position(),
            Self::Subtraction { left, .. } => left.initial_position(),
            Self::Multiplication { left, .. } => left.initial_position(),
            Self::Division { left, .. } => left.initial_position(),
            Self::Modulo { left, .. } => left.initial_position(),
            Self::Exponentiation { left, .. } => left.initial_position(),
            Self::BitwiseAnd { left, .. } => left.initial_position(),
            Self::BitwiseOr { left, .. } => left.initial_position(),
            Self::BitwiseXor { left, .. } => left.initial_position(),
            Self::LeftShift { left, .. } => left.initial_position(),
            Self::RightShift { left, .. } => left.initial_position(),
            Self::Coalesce { left, .. } => left.initial_position(),
            Self::Concat { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Assignment { right, .. } => right.final_position(),
            Self::Addition { right, .. } => right.final_position(),
            Self::Subtraction { right, .. } => right.final_position(),
            Self::Multiplication { right, .. } => right.final_position(),
            Self::Division { right, .. } => right.final_position(),
            Self::Modulo { right, .. } => right.final_position(),
            Self::Exponentiation { right, .. } => right.final_position(),
            Self::BitwiseAnd { right, .. } => right.final_position(),
            Self::BitwiseOr { right, .. } => right.final_position(),
            Self::BitwiseXor { right, .. } => right.final_position(),
            Self::LeftShift { right, .. } => right.final_position(),
            Self::RightShift { right, .. } => right.final_position(),
            Self::Coalesce { right, .. } => right.final_position(),
            Self::Concat { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Assignment { left, right, .. }
            | Self::Addition { left, right, .. }
            | Self::Subtraction { left, right, .. }
            | Self::Multiplication { left, right, .. }
            | Self::Division { left, right, .. }
            | Self::Modulo { left, right, .. }
            | Self::Exponentiation { left, right, .. }
            | Self::BitwiseAnd { left, right, .. }
            | Self::BitwiseOr { left, right, .. }
            | Self::BitwiseXor { left, right, .. }
            | Self::LeftShift { left, right, .. }
            | Self::RightShift { left, right, .. }
            | Self::Coalesce { left, right, .. }
            | Self::Concat { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Assignment { .. } => "assignment operation expression".to_string(),
            Self::Addition { .. } => "addition assignment operation expression".to_string(),
            Self::Subtraction { .. } => "subtraction assignment operation expression".to_string(),
            Self::Multiplication { .. } => {
                "multiplication assignment operation expression".to_string()
            }
            Self::Division { .. } => "division assignment operation expression".to_string(),
            Self::Modulo { .. } => "modulo assignment operation expression".to_string(),
            Self::Exponentiation { .. } => {
                "exponentiation assignment operation expression".to_string()
            }
            Self::BitwiseAnd { .. } => "bitwise AND assignment operation expression".to_string(),
            Self::BitwiseOr { .. } => "bitwise OR assignment operation expression".to_string(),
            Self::BitwiseXor { .. } => "bitwise XOR assignment operation expression".to_string(),
            Self::LeftShift { .. } => "left shift assignment operation expression".to_string(),
            Self::RightShift { .. } => "right shift assignment operation expression".to_string(),
            Self::Coalesce { .. } => "coalesce assignment operation expression".to_string(),
            Self::Concat { .. } => "concat assignment operation expression".to_string(),
        }
    }
}

impl Node for BitwiseOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::And { comments, .. } => Some(comments),
            Self::Or { comments, .. } => Some(comments),
            Self::Xor { comments, .. } => Some(comments),
            Self::LeftShift { comments, .. } => Some(comments),
            Self::RightShift { comments, .. } => Some(comments),
            Self::Not { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::And { left, .. } => left.initial_position(),
            Self::Or { left, .. } => left.initial_position(),
            Self::Xor { left, .. } => left.initial_position(),
            Self::LeftShift { left, .. } => left.initial_position(),
            Self::RightShift { left, .. } => left.initial_position(),
            Self::Not { not, .. } => *not,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::And { right, .. } => right.final_position(),
            Self::Or { right, .. } => right.final_position(),
            Self::Xor { right, .. } => right.final_position(),
            Self::LeftShift { right, .. } => right.final_position(),
            Self::RightShift { right, .. } => right.final_position(),
            Self::Not { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::And { left, right, .. }
            | Self::Or { left, right, .. }
            | Self::Xor { left, right, .. }
            | Self::LeftShift { left, right, .. }
            | Self::RightShift { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
            Self::Not { right, .. } => vec![right.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::And { .. } => "bitwise AND operation expression".to_string(),
            Self::Or { .. } => "bitwise OR operation expression".to_string(),
            Self::Xor { .. } => "bitwise XOR operation expression".to_string(),
            Self::LeftShift { .. } => "left shift operation expression".to_string(),
            Self::RightShift { .. } => "right shift operation expression".to_string(),
            Self::Not { .. } => "bitwise NOT operation expression".to_string(),
        }
    }
}

impl Node for ComparisonOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Equal { comments, .. }
            | Self::NotEqual { comments, .. }
            | Self::Identical { comments, .. }
            | Self::NotIdentical { comments, .. }
            | Self::LessThan { comments, .. }
            | Self::LessThanOrEqual { comments, .. }
            | Self::GreaterThan { comments, .. }
            | Self::GreaterThanOrEqual { comments, .. }
            | Self::Spaceship { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Equal { left, .. }
            | Self::NotEqual { left, .. }
            | Self::Identical { left, .. }
            | Self::NotIdentical { left, .. }
            | Self::LessThan { left, .. }
            | Self::LessThanOrEqual { left, .. }
            | Self::GreaterThan { left, .. }
            | Self::GreaterThanOrEqual { left, .. }
            | Self::Spaceship { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Equal { right, .. }
            | Self::NotEqual { right, .. }
            | Self::Identical { right, .. }
            | Self::NotIdentical { right, .. }
            | Self::LessThan { right, .. }
            | Self::LessThanOrEqual { right, .. }
            | Self::GreaterThan { right, .. }
            | Self::GreaterThanOrEqual { right, .. }
            | Self::Spaceship { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Equal { left, right, .. }
            | Self::NotEqual { left, right, .. }
            | Self::Identical { left, right, .. }
            | Self::NotIdentical { left, right, .. }
            | Self::LessThan { left, right, .. }
            | Self::LessThanOrEqual { left, right, .. }
            | Self::GreaterThan { left, right, .. }
            | Self::GreaterThanOrEqual { left, right, .. }
            | Self::Spaceship { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Equal { .. } => "equal comparison operation expression".to_string(),
            Self::NotEqual { .. } => "not equal comparison operation expression".to_string(),
            Self::Identical { .. } => "identical comparison operation expression".to_string(),
            Self::NotIdentical { .. } => {
                "not identical comparison operation expression".to_string()
            }
            Self::LessThan { .. } => "less than comparison operation expression".to_string(),
            Self::LessThanOrEqual { .. } => {
                "less than or equal comparison operation expression".to_string()
            }
            Self::GreaterThan { .. } => "greater than comparison operation expression".to_string(),
            Self::GreaterThanOrEqual { .. } => {
                "greater than or equal comparison operation expression".to_string()
            }
            Self::Spaceship { .. } => "spaceship comparison operation expression".to_string(),
        }
    }
}

impl Node for LogicalOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::And { comments, .. } => Some(comments),
            Self::Or { comments, .. } => Some(comments),
            Self::Not { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::And { left, .. } => left.initial_position(),
            Self::Or { left, .. } => left.initial_position(),
            Self::Not { bang, .. } => *bang,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::And { right, .. } => right.final_position(),
            Self::Or { right, .. } => right.final_position(),
            Self::Not { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::And { left, right, .. } | Self::Or { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
            Self::Not { right, .. } => vec![right.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::And { .. } => "logical AND operation expression".to_string(),
            Self::Or { .. } => "logical OR operation expression".to_string(),
            Self::Not { .. } => "logical NOT operation expression".to_string(),
        }
    }
}

impl Node for StringOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Concat { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Concat { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Concat { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Concat { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Concat { .. } => "string concatenation operation expression".to_string(),
        }
    }
}

impl Node for ArrayOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Access { comments, .. }
            | Self::Push { comments, .. }
            | Self::Isset { comments, .. }
            | Self::Unset { comments, .. }
            | Self::In { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Access { array, .. } | Self::Push { array, .. } => array.initial_position(),
            Self::Isset { isset, .. } => isset.initial_position(),
            Self::Unset { unset, .. } => unset.initial_position(),
            Self::In { item, .. } => item.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Access { right_bracket, .. } | Self::Push { right_bracket, .. } => {
                right_bracket + 1
            }
            Self::Isset { item, .. } | Self::Unset { item, .. } => item.final_position(),
            Self::In { array, .. } => array.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Access { array, index, .. } => {
                vec![array.as_ref(), index.as_ref()]
            }
            Self::Push { array, .. } => {
                vec![array.as_ref()]
            }
            Self::Isset {
                isset: keyword,
                item,
                ..
            }
            | Self::Unset {
                unset: keyword,
                item,
                ..
            } => {
                vec![keyword, item.as_ref()]
            }
            Self::In {
                item, r#in, array, ..
            } => {
                vec![item.as_ref(), r#in, array.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Access { .. } => "array access operation expression".to_string(),
            Self::Push { .. } => "array push operation expression".to_string(),
            Self::Isset { .. } => "array isset operation expression".to_string(),
            Self::Unset { .. } => "array unset operation expression".to_string(),
            Self::In { .. } => "array in operation expression".to_string(),
        }
    }
}

impl Node for CoalesceOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Coalesce { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Coalesce { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Coalesce { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Coalesce { left, right, .. } => {
                vec![left.as_ref(), right.as_ref()]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Coalesce { .. } => "coalesce operation expression".to_string(),
        }
    }
}

impl Node for TernaryOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Ternary { comments, .. } => Some(comments),
            Self::ShortTernary { comments, .. } => Some(comments),
            Self::ImplicitShortTernary { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Ternary { condition, .. } => condition.initial_position(),
            Self::ShortTernary { condition, .. } => condition.initial_position(),
            Self::ImplicitShortTernary { condition, .. } => condition.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Ternary { if_false, .. } => if_false.final_position(),
            Self::ShortTernary { if_false, .. } => if_false.final_position(),
            Self::ImplicitShortTernary { if_false, .. } => if_false.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Ternary {
                condition,
                if_true,
                if_false,
                ..
            } => vec![condition.as_ref(), if_true.as_ref(), if_false.as_ref()],
            Self::ShortTernary {
                condition,
                if_false,
                ..
            } => vec![condition.as_ref(), if_false.as_ref()],
            Self::ImplicitShortTernary {
                condition,
                if_false,
                ..
            } => vec![condition.as_ref(), if_false.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Ternary { .. } => "ternary operation expression".to_string(),
            Self::ShortTernary { .. } => "short ternary operation expression".to_string(),
            Self::ImplicitShortTernary { .. } => {
                "implicit short ternary operation expression".to_string()
            }
        }
    }
}

impl Node for TypeOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Instanceof { comments, .. } => Some(comments),
            Self::Is { comments, .. } => Some(comments),
            Self::Into { comments, .. } => Some(comments),
            Self::As { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Instanceof { left, .. } => left.initial_position(),
            Self::Is { left, .. } => left.initial_position(),
            Self::Into { left, .. } => left.initial_position(),
            Self::As { left, .. } => left.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Instanceof { right, .. } => right.final_position(),
            Self::Is { right, .. } => right.final_position(),
            Self::Into { right, .. } => right.final_position(),
            Self::As { right, .. } => right.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Instanceof {
                left,
                instanceof,
                right,
                ..
            } => vec![left.as_ref(), instanceof, right],
            Self::Is {
                left, is, right, ..
            } => vec![left.as_ref(), is, right],
            Self::Into {
                left, into, right, ..
            } => vec![left.as_ref(), into, right],
            Self::As {
                left, r#as, right, ..
            } => vec![left.as_ref(), r#as, right],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Instanceof { .. } => "instanceof type operation expression".to_string(),
            Self::Is { .. } => "is type operation expression".to_string(),
            Self::Into { .. } => "into type operation expression".to_string(),
            Self::As { .. } => "as type operation expression".to_string(),
        }
    }
}

impl Node for GeneratorOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Yield { comments, .. } => Some(comments),
            Self::YieldValue { comments, .. } => Some(comments),
            Self::YieldKeyValue { comments, .. } => Some(comments),
            Self::YieldFrom { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Yield { r#yield, .. }
            | Self::YieldValue { r#yield, .. }
            | Self::YieldKeyValue { r#yield, .. }
            | Self::YieldFrom { r#yield, .. } => r#yield.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Yield { r#yield, .. } => r#yield.final_position(),
            Self::YieldValue { value, .. } => value.final_position(),
            Self::YieldKeyValue { value, .. } => value.final_position(),
            Self::YieldFrom { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Yield { r#yield, .. } => vec![r#yield],
            Self::YieldValue { r#yield, value, .. } => {
                vec![r#yield, value.as_ref()]
            }
            Self::YieldKeyValue {
                r#yield,
                key,
                value,
                ..
            } => {
                vec![r#yield, key.as_ref(), value.as_ref()]
            }
            Self::YieldFrom {
                r#yield,
                from,
                value,
                ..
            } => vec![r#yield, from, value.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Yield { .. } => "yield generator operation expression".to_string(),
            Self::YieldValue { .. } => "yield value generator operation expression".to_string(),
            Self::YieldKeyValue { .. } => {
                "yield key value generator operation expression".to_string()
            }
            Self::YieldFrom { .. } => "yield from generator operation expression".to_string(),
        }
    }
}

impl Node for ExceptionOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Throw { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Throw { throw, .. } => throw.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Throw { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Throw { throw, value, .. } => vec![throw, value.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Throw { .. } => "throw exception operation expression".to_string(),
        }
    }
}

impl Node for ObjectOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Clone { comments, .. } => Some(comments),
            Self::MethodCall { comments, .. } => Some(comments),
            Self::NullsafeMethodCall { comments, .. } => Some(comments),
            Self::MethodClosureCreation { comments, .. } => Some(comments),
            Self::PropertyFetch { comments, .. } => Some(comments),
            Self::NullsafePropertyFetch { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Clone { clone, .. } => clone.initial_position(),
            Self::MethodCall { object, .. } => object.initial_position(),
            Self::NullsafeMethodCall { object, .. } => object.initial_position(),
            Self::MethodClosureCreation { object, .. } => object.initial_position(),
            Self::PropertyFetch { object, .. } => object.initial_position(),
            Self::NullsafePropertyFetch { object, .. } => object.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Clone { object, .. } => object.final_position(),
            Self::MethodCall { arguments, .. } => arguments.final_position(),
            Self::NullsafeMethodCall { arguments, .. } => arguments.final_position(),
            Self::MethodClosureCreation { placeholder, .. } => placeholder.final_position(),
            Self::PropertyFetch { property, .. } => property.final_position(),
            Self::NullsafePropertyFetch { property, .. } => property.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Clone { clone, object, .. } => vec![clone, object.as_ref()],
            Self::MethodCall {
                object,
                method,
                generics,
                arguments,
                ..
            }
            | Self::NullsafeMethodCall {
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
            Self::MethodClosureCreation {
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
            Self::PropertyFetch {
                object, property, ..
            }
            | Self::NullsafePropertyFetch {
                object, property, ..
            } => {
                vec![object.as_ref(), property]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Clone { .. } => "object clone operation expression".to_string(),
            Self::MethodCall { .. } => "object method call operation expression".to_string(),
            Self::NullsafeMethodCall { .. } => {
                "object nullsafe method call operation expression".to_string()
            }
            Self::MethodClosureCreation { .. } => {
                "object method closure creation operation expression".to_string()
            }
            Self::PropertyFetch { .. } => "object property fetch operation expression".to_string(),
            Self::NullsafePropertyFetch { .. } => {
                "object nullsafe property fetch operation expression".to_string()
            }
        }
    }
}

impl Node for ClassOperationInitializationClassExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Identifier(_) => None,
            Self::Variable(_) => None,
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Identifier(identifier) => identifier.initial_position(),
            Self::Variable(variable) => variable.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Identifier(identifier) => identifier.final_position(),
            Self::Variable(variable) => variable.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Identifier(identifier) => vec![identifier],
            Self::Variable(variable) => vec![variable],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Identifier(identifier) => identifier.get_description(),
            Self::Variable(variable) => variable.get_description(),
        }
    }
}

impl Node for ClassOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Initialization { comments, .. } => Some(comments),
            Self::AnonymousInitialization { comments, .. } => Some(comments),
            Self::StaticMethodCall { comments, .. } => Some(comments),
            Self::StaticMethodClosureCreation { comments, .. } => Some(comments),
            Self::StaticPropertyFetch { comments, .. } => Some(comments),
            Self::ConstantFetch { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Initialization { new, .. }
            | ClassOperationExpression::AnonymousInitialization { new, .. } => {
                new.initial_position()
            }
            Self::StaticMethodCall { class, .. } => class.initial_position(),
            Self::StaticMethodClosureCreation { class, .. } => class.initial_position(),
            Self::StaticPropertyFetch { class, .. } => class.initial_position(),
            Self::ConstantFetch { class, .. } => class.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Initialization { arguments, .. } => arguments.final_position(),
            Self::AnonymousInitialization { class, .. } => class.final_position(),
            Self::StaticMethodCall { arguments, .. } => arguments.final_position(),
            Self::StaticMethodClosureCreation { placeholder, .. } => placeholder.final_position(),
            Self::StaticPropertyFetch { property, .. } => property.final_position(),
            Self::ConstantFetch { constant, .. } => constant.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Initialization {
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
            Self::AnonymousInitialization { new, class, .. } => {
                vec![new, class]
            }
            Self::StaticMethodCall {
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
            Self::StaticMethodClosureCreation {
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
            Self::StaticPropertyFetch {
                class, property, ..
            } => {
                vec![class.as_ref(), property]
            }
            Self::ConstantFetch {
                class, constant, ..
            } => {
                vec![class.as_ref(), constant]
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Initialization { .. } => "class initialization operation expression".to_string(),
            Self::AnonymousInitialization { .. } => {
                "class anonymous initialization operation expression".to_string()
            }
            Self::StaticMethodCall { .. } => {
                "class static method call operation expression".to_string()
            }
            Self::StaticMethodClosureCreation { .. } => {
                "class static method closure creation operation expression".to_string()
            }
            Self::StaticPropertyFetch { .. } => {
                "class static property fetch operation expression".to_string()
            }
            Self::ConstantFetch { .. } => "class constant fetch operation expression".to_string(),
        }
    }
}

impl Node for FunctionOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Call { comments, .. } => Some(comments),
            Self::ClosureCreation { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Call { function, .. } => function.initial_position(),
            Self::ClosureCreation { function, .. } => function.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Call { arguments, .. } => arguments.final_position(),
            Self::ClosureCreation { placeholder, .. } => placeholder.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Call {
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
            Self::ClosureCreation {
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
        match &self {
            Self::Call { .. } => "function call operation expression".to_string(),
            Self::ClosureCreation { .. } => {
                "function closure creation operation expression".to_string()
            }
        }
    }
}

impl Node for AsyncOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Await { comments, .. } => Some(comments),
            Self::Async { comments, .. } => Some(comments),
            Self::Concurrently { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Await { r#await, .. } => r#await.initial_position(),
            Self::Async { r#async, .. } => r#async.initial_position(),
            Self::Concurrently { concurrently, .. } => concurrently.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Await { expression, .. } => expression.final_position(),
            Self::Async { expression, .. } => expression.final_position(),
            Self::Concurrently { right_brace, .. } => right_brace + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Await {
                r#await,
                expression,
                ..
            } => vec![r#await, expression.as_ref()],
            Self::Async {
                r#async,
                expression,
                ..
            } => vec![r#async, expression.as_ref()],
            Self::Concurrently {
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
        match &self {
            Self::Await { .. } => "async await operation expression".to_string(),
            Self::Async { .. } => "async operation expression".to_string(),
            Self::Concurrently { .. } => "async concurrently operation expression".to_string(),
        }
    }
}

impl Node for RangeOperationExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Self::Between { comments, .. } => Some(comments),
            Self::BetweenInclusive { comments, .. } => Some(comments),
            Self::To { comments, .. } => Some(comments),
            Self::ToInclusive { comments, .. } => Some(comments),
            Self::From { comments, .. } => Some(comments),
            Self::Full { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::Between { from, .. } => from.initial_position(),
            Self::BetweenInclusive { from, .. } => from.initial_position(),
            Self::To { double_dot, .. } => *double_dot,
            Self::ToInclusive { double_dot, .. } => *double_dot,
            Self::From { from, .. } => from.initial_position(),
            Self::Full { double_dot, .. } => *double_dot,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Between { to, .. } => to.final_position(),
            Self::BetweenInclusive { to, .. } => to.final_position(),
            Self::To { to, .. } => to.final_position(),
            Self::ToInclusive { to, .. } => to.final_position(),
            Self::From { double_dot, .. } => *double_dot,
            Self::Full { double_dot, .. } => *double_dot,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Between { from, to, .. } => {
                vec![from.as_ref(), to.as_ref()]
            }
            Self::BetweenInclusive { from, to, .. } => {
                vec![from.as_ref(), to.as_ref()]
            }
            Self::To { to, .. } => vec![to.as_ref()],
            Self::ToInclusive { to, .. } => vec![to.as_ref()],
            Self::From { from, .. } => vec![from.as_ref()],
            Self::Full { .. } => vec![],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Between { .. } => "range between operation expression".to_string(),
            Self::BetweenInclusive { .. } => {
                "range between inclusive operation expression".to_string()
            }
            Self::To { .. } => "range to operation expression".to_string(),
            Self::ToInclusive { .. } => "range to inclusive operation expression".to_string(),
            Self::From { .. } => "range from operation expression".to_string(),
            Self::Full { .. } => "range full operation expression".to_string(),
        }
    }
}

impl std::fmt::Display for ArithmeticOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Addition { left, right, .. } => {
                write!(f, "{} + {}", left, right)
            }
            Self::Subtraction { left, right, .. } => {
                write!(f, "{} - {}", left, right)
            }
            Self::Multiplication { left, right, .. } => {
                write!(f, "{} * {}", left, right)
            }
            Self::Division { left, right, .. } => {
                write!(f, "{} / {}", left, right)
            }
            Self::Exponentiation { left, right, .. } => {
                write!(f, "{} ** {}", left, right)
            }
            Self::Modulo { left, right, .. } => {
                write!(f, "{} % {}", left, right)
            }
            Self::Negative { right, .. } => {
                write!(f, "-{}", right)
            }
            Self::Positive { right, .. } => {
                write!(f, "+{}", right)
            }
            Self::PostDecrement { left, .. } => {
                write!(f, "{}--", left)
            }
            Self::PostIncrement { left, .. } => {
                write!(f, "{}++", left)
            }
            Self::PreDecrement { right, .. } => {
                write!(f, "--{}", right)
            }
            Self::PreIncrement { right, .. } => {
                write!(f, "++{}", right)
            }
        }
    }
}

impl std::fmt::Display for ComparisonOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Equal { left, right, .. } => {
                write!(f, "{} == {}", left, right)
            }
            Self::Identical { left, right, .. } => {
                write!(f, "{} === {}", left, right)
            }
            Self::NotIdentical { left, right, .. } => {
                write!(f, "{} !== {}", left, right)
            }
            Self::NotEqual { left, right, .. } => {
                write!(f, "{} != {}", left, right)
            }
            Self::LessThan { left, right, .. } => {
                write!(f, "{} < {}", left, right)
            }
            Self::LessThanOrEqual { left, right, .. } => {
                write!(f, "{} <= {}", left, right)
            }
            Self::GreaterThan { left, right, .. } => {
                write!(f, "{} > {}", left, right)
            }
            Self::GreaterThanOrEqual { left, right, .. } => {
                write!(f, "{} >= {}", left, right)
            }
            Self::Spaceship { left, right, .. } => {
                write!(f, "{} <=> {}", left, right)
            }
        }
    }
}

impl std::fmt::Display for LogicalOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::And { left, right, .. } => {
                write!(f, "{} && {}", left, right)
            }
            Self::Or { left, right, .. } => {
                write!(f, "{} || {}", left, right)
            }
            Self::Not { right, .. } => {
                write!(f, "!{}", right)
            }
        }
    }
}

impl std::fmt::Display for AssignmentOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Assignment { left, right, .. } => {
                write!(f, "{} = {}", left, right)
            }
            Self::Addition { left, right, .. } => {
                write!(f, "{} += {}", left, right)
            }
            Self::Subtraction { left, right, .. } => {
                write!(f, "{} -= {}", left, right)
            }
            Self::Multiplication { left, right, .. } => {
                write!(f, "{} *= {}", left, right)
            }
            Self::Division { left, right, .. } => {
                write!(f, "{} /= {}", left, right)
            }
            Self::Exponentiation { left, right, .. } => {
                write!(f, "{} **= {}", left, right)
            }
            Self::Modulo { left, right, .. } => {
                write!(f, "{} %= {}", left, right)
            }
            Self::Concat { left, right, .. } => {
                write!(f, "{} .= {}", left, right)
            }
            Self::BitwiseAnd { left, right, .. } => {
                write!(f, "{} &= {}", left, right)
            }
            Self::BitwiseOr { left, right, .. } => {
                write!(f, "{} |= {}", left, right)
            }
            Self::BitwiseXor { left, right, .. } => {
                write!(f, "{} ^= {}", left, right)
            }
            Self::LeftShift { left, right, .. } => {
                write!(f, "{} <<= {}", left, right)
            }
            Self::RightShift { left, right, .. } => {
                write!(f, "{} >>= {}", left, right)
            }
            Self::Coalesce { left, right, .. } => {
                write!(f, "{} ??= {}", left, right)
            }
        }
    }
}

impl std::fmt::Display for BitwiseOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::LeftShift { left, right, .. } => {
                write!(f, "{} << {}", left, right)
            }
            Self::RightShift { left, right, .. } => {
                write!(f, "{} >> {}", left, right)
            }
            Self::And { left, right, .. } => {
                write!(f, "{} & {}", left, right)
            }
            Self::Or { left, right, .. } => {
                write!(f, "{} | {}", left, right)
            }
            Self::Xor { left, right, .. } => {
                write!(f, "{} ^ {}", left, right)
            }
            Self::Not { right, .. } => {
                write!(f, "~{}", right)
            }
        }
    }
}

impl std::fmt::Display for AsyncOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Await {
                r#await,
                expression,
                ..
            } => {
                write!(f, "{} {}", r#await, expression)
            }
            Self::Async {
                r#async,
                expression,
                ..
            } => {
                write!(f, "{} {}", r#async, expression)
            }
            Self::Concurrently { concurrently, .. } => {
                write!(f, "{} {{{{ /* ... */ }}}}", concurrently)
            }
        }
    }
}

impl std::fmt::Display for RangeOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Between { from, to, .. } => {
                write!(f, "{}..{}", from, to)
            }
            Self::BetweenInclusive { from, to, .. } => {
                write!(f, "{}..={}", from, to)
            }
            Self::To { to, .. } => {
                write!(f, "..{}", to)
            }
            Self::ToInclusive { to, .. } => {
                write!(f, "..={}", to)
            }
            Self::From { from, .. } => {
                write!(f, "{}..", from)
            }
            Self::Full { .. } => {
                write!(f, "..")
            }
        }
    }
}

impl std::fmt::Display for FunctionalOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Expression {
                generics,
                expression,
                ..
            } => {
                write!(f, "$(")?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", expression)?;
                write!(f, ")")
            }
            Self::Pipe { left, right, .. } => write!(f, "{} |> {}", left, right),
        }
    }
}

impl std::fmt::Display for StringOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Concat { left, right, .. } => {
                write!(f, "{}.{}", left, right)
            }
        }
    }
}

impl std::fmt::Display for ArrayOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Access { array, index, .. } => {
                write!(f, "{}[{}]", array, index)
            }
            Self::Push { array, .. } => {
                write!(f, "{}[]", array)
            }
            Self::Unset { item, .. } => {
                write!(f, "unset {}", item)
            }
            Self::Isset { item, .. } => {
                write!(f, "isset {}", item)
            }
            Self::In { item, array, .. } => {
                write!(f, "{} in {}", item, array)
            }
        }
    }
}

impl std::fmt::Display for CoalesceOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Coalesce { left, right, .. } => {
                write!(f, "{} ?? {}", left, right)
            }
        }
    }
}

impl std::fmt::Display for TernaryOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Ternary {
                condition,
                if_true,
                if_false,
                ..
            } => {
                write!(f, "{} ? {} : {}", condition, if_true, if_false)
            }
            Self::ShortTernary {
                condition,
                if_false,
                ..
            } => {
                write!(f, "{} ?: {}", condition, if_false)
            }
            Self::ImplicitShortTernary {
                condition,
                if_false,
                ..
            } => {
                write!(f, "{} ? : {}", condition, if_false)
            }
        }
    }
}

impl std::fmt::Display for TypeOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Instanceof { left, right, .. } => {
                write!(f, "{} instanceof {}", left, right)
            }
            Self::Is { left, right, .. } => {
                write!(f, "{} is {}", left, right)
            }
            Self::Into { left, right, .. } => {
                write!(f, "{} into {}", left, right)
            }
            Self::As { left, right, .. } => {
                write!(f, "{} as {}", left, right)
            }
        }
    }
}

impl std::fmt::Display for GeneratorOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Yield { .. } => {
                write!(f, "yield")
            }
            Self::YieldFrom { value, .. } => {
                write!(f, "yield from {}", value)
            }
            Self::YieldValue { value, .. } => {
                write!(f, "yield {}", value)
            }
            Self::YieldKeyValue { key, value, .. } => {
                write!(f, "yield {} => {}", key, value)
            }
        }
    }
}

impl std::fmt::Display for ExceptionOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Throw { value, .. } => {
                write!(f, "throw {}", value)
            }
        }
    }
}

impl std::fmt::Display for ObjectOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Clone { object, .. } => {
                write!(f, "clone {}", object)
            }
            Self::MethodCall {
                object,
                method,
                generics,
                arguments,
                ..
            } => {
                write!(f, "{}->{}", object, method)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", arguments)
            }
            Self::NullsafeMethodCall {
                object,
                method,
                generics,
                arguments,
                ..
            } => {
                write!(f, "{}?->{}", object, method)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", arguments)
            }
            Self::MethodClosureCreation {
                object,
                method,
                generics,
                placeholder,
                ..
            } => {
                write!(f, "{}->{}", object, method)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", placeholder)
            }
            Self::PropertyFetch {
                object, property, ..
            } => {
                write!(f, "{}->{}", object, property)
            }
            Self::NullsafePropertyFetch {
                object, property, ..
            } => {
                write!(f, "{}?->{}", object, property)
            }
        }
    }
}

impl std::fmt::Display for ClassOperationInitializationClassExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Identifier(identifier) => {
                write!(f, "new {}", identifier)
            }
            Self::Variable(variable) => {
                write!(f, "new {}", variable)
            }
        }
    }
}

impl std::fmt::Display for ClassOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Initialization {
                class,
                generics,
                arguments,
                ..
            } => {
                write!(f, "{}", class)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", arguments)
            }
            Self::AnonymousInitialization { class, .. } => write!(f, "new {}", class),
            Self::StaticMethodCall {
                class,
                method,
                generics,
                arguments,
                ..
            } => {
                write!(f, "{}::{}", class, method)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", arguments)
            }
            Self::StaticMethodClosureCreation {
                class,
                method,
                generics,
                placeholder,
                ..
            } => {
                write!(f, "{}::{}", class, method)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", placeholder)
            }
            Self::StaticPropertyFetch {
                class, property, ..
            } => {
                write!(f, "{}::{}", class, property)
            }
            Self::ConstantFetch {
                class, constant, ..
            } => {
                write!(f, "{}::{}", class, constant)
            }
        }
    }
}

impl std::fmt::Display for FunctionOperationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Call {
                function,
                generics,
                arguments,
                ..
            } => {
                write!(f, "{}", function)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", arguments)
            }
            Self::ClosureCreation {
                function,
                generics,
                placeholder,
                ..
            } => {
                write!(f, "{}", function)?;
                if let Some(generics) = generics {
                    write!(f, "{}", generics)?;
                }
                write!(f, "{}", placeholder)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::definition::class::ClassDefinitionBody;
    use crate::tree::definition::r#type::SignedIntegerTypeDefinition;
    use crate::tree::expression::argument::ArgumentExpression;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;

    #[test]
    fn test_functional_operation_expression_display() {
        let pipe = FunctionalOperationExpression::Pipe {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            pipe: 0,
            greater_than: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(pipe.to_string(), "$foo |> $bar");

        let expression = FunctionalOperationExpression::Expression {
            comments: CommentGroup { comments: vec![] },
            dollar: 0,
            generics: None,
            left_parenthesis: 0,
            expression: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right_parenthesis: 0,
        };

        assert_eq!(expression.to_string(), "$($foo)");
    }

    #[test]
    fn test_assignment_operation_expression_display() {
        let assignment = AssignmentOperationExpression::Assignment {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            equals: 0,
        };

        assert_eq!(assignment.to_string(), "$foo = $bar");

        let addition = AssignmentOperationExpression::Addition {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            plus_equals: 0,
        };

        assert_eq!(addition.to_string(), "$foo += $bar");

        let subtraction = AssignmentOperationExpression::Subtraction {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            minus_equals: 0,
        };

        assert_eq!(subtraction.to_string(), "$foo -= $bar");

        let multiplication = AssignmentOperationExpression::Multiplication {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            asterisk_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(multiplication.to_string(), "$foo *= $bar");

        let division = AssignmentOperationExpression::Division {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            slash_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(division.to_string(), "$foo /= $bar");

        let modulo = AssignmentOperationExpression::Modulo {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            percent_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(modulo.to_string(), "$foo %= $bar");

        let bitwise_and = AssignmentOperationExpression::BitwiseAnd {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            ampersand_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(bitwise_and.to_string(), "$foo &= $bar");

        let bitwise_or = AssignmentOperationExpression::BitwiseOr {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            pipe_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(bitwise_or.to_string(), "$foo |= $bar");

        let bitwise_xor = AssignmentOperationExpression::BitwiseXor {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            caret_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(bitwise_xor.to_string(), "$foo ^= $bar");

        let left_shift = AssignmentOperationExpression::LeftShift {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            left_shift_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(left_shift.to_string(), "$foo <<= $bar");

        let right_shift = AssignmentOperationExpression::RightShift {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right_shift_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(right_shift.to_string(), "$foo >>= $bar");

        let exponentiation = AssignmentOperationExpression::Exponentiation {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            pow_equals: 0,
        };

        assert_eq!(exponentiation.to_string(), "$foo **= $bar");

        let concat = AssignmentOperationExpression::Concat {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            dot_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(concat.to_string(), "$foo .= $bar");

        let coalesce = AssignmentOperationExpression::Coalesce {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            coalesce_equals: 0,
        };

        assert_eq!(coalesce.to_string(), "$foo ??= $bar");
    }

    #[test]
    fn test_arithmetic_operation_expression_display() {
        let addition = ArithmeticOperationExpression::Addition {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            plus: 0,
        };

        assert_eq!(addition.to_string(), "$foo + $bar");

        let subtraction = ArithmeticOperationExpression::Subtraction {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            minus: 0,
        };

        assert_eq!(subtraction.to_string(), "$foo - $bar");

        let multiplication = ArithmeticOperationExpression::Multiplication {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            asterisk: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(multiplication.to_string(), "$foo * $bar");

        let division = ArithmeticOperationExpression::Division {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            slash: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(division.to_string(), "$foo / $bar");

        let modulo = ArithmeticOperationExpression::Modulo {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            percent: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(modulo.to_string(), "$foo % $bar");

        let exponentiation = ArithmeticOperationExpression::Exponentiation {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            pow: 0,
        };

        assert_eq!(exponentiation.to_string(), "$foo ** $bar");

        let negative = ArithmeticOperationExpression::Negative {
            comments: CommentGroup { comments: vec![] },
            minus: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(negative.to_string(), "-$bar");

        let positive = ArithmeticOperationExpression::Positive {
            comments: CommentGroup { comments: vec![] },
            plus: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(positive.to_string(), "+$bar");

        let pre_increment = ArithmeticOperationExpression::PreIncrement {
            comments: CommentGroup { comments: vec![] },
            increment: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(pre_increment.to_string(), "++$bar");

        let pre_decrement = ArithmeticOperationExpression::PreDecrement {
            comments: CommentGroup { comments: vec![] },
            decrement: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(pre_decrement.to_string(), "--$bar");

        let post_increment = ArithmeticOperationExpression::PostIncrement {
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            increment: 0,
        };

        assert_eq!(post_increment.to_string(), "$bar++");

        let post_decrement = ArithmeticOperationExpression::PostDecrement {
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            decrement: 0,
        };

        assert_eq!(post_decrement.to_string(), "$bar--");
    }

    #[test]
    fn test_bitwise_operation_expression_display() {
        let bitwise_and = BitwiseOperationExpression::And {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            and: 0,
        };

        assert_eq!(bitwise_and.to_string(), "$foo & $bar");

        let bitwise_or = BitwiseOperationExpression::Or {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            or: 0,
        };

        assert_eq!(bitwise_or.to_string(), "$foo | $bar");

        let bitwise_xor = BitwiseOperationExpression::Xor {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            xor: 0,
        };

        assert_eq!(bitwise_xor.to_string(), "$foo ^ $bar");

        let bitwise_not = BitwiseOperationExpression::Not {
            comments: CommentGroup { comments: vec![] },
            not: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(bitwise_not.to_string(), "~$bar");

        let bitwise_left_shift = BitwiseOperationExpression::LeftShift {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            left_shift: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(bitwise_left_shift.to_string(), "$foo << $bar");

        let bitwise_right_shift = BitwiseOperationExpression::RightShift {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right_shift: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(bitwise_right_shift.to_string(), "$foo >> $bar");
    }

    #[test]
    fn test_comparison_operation_expression_display() {
        let equal = ComparisonOperationExpression::Equal {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            double_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(equal.to_string(), "$foo == $bar");

        let identical = ComparisonOperationExpression::Identical {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            triple_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(identical.to_string(), "$foo === $bar");

        let not_equal = ComparisonOperationExpression::NotEqual {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            bang_equals: 0,
        };

        assert_eq!(not_equal.to_string(), "$foo != $bar");

        let not_identical = ComparisonOperationExpression::NotIdentical {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            bang_double_equals: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(not_identical.to_string(), "$foo !== $bar");

        let less_than = ComparisonOperationExpression::LessThan {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            less_than: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(less_than.to_string(), "$foo < $bar");

        let greater_than = ComparisonOperationExpression::GreaterThan {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            greater_than: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(greater_than.to_string(), "$foo > $bar");

        let less_than_or_equal = ComparisonOperationExpression::LessThanOrEqual {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            less_than_equals: 0,
        };

        assert_eq!(less_than_or_equal.to_string(), "$foo <= $bar");

        let greater_than_or_equal = ComparisonOperationExpression::GreaterThanOrEqual {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            greater_than_equals: 0,
        };

        assert_eq!(greater_than_or_equal.to_string(), "$foo >= $bar");

        let spaceship = ComparisonOperationExpression::Spaceship {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            spaceship: 0,
        };

        assert_eq!(spaceship.to_string(), "$foo <=> $bar");
    }

    #[test]
    fn test_logical_operation_expression_display() {
        let logical_and = LogicalOperationExpression::And {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            double_ampersand: 0,
        };

        assert_eq!(logical_and.to_string(), "$foo && $bar");

        let logical_or = LogicalOperationExpression::Or {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            double_pipe: 0,
        };

        assert_eq!(logical_or.to_string(), "$foo || $bar");

        let logical_not = LogicalOperationExpression::Not {
            comments: CommentGroup { comments: vec![] },
            bang: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(logical_not.to_string(), "!$foo");
    }

    #[test]
    fn test_string_operation_expression_display() {
        let concat = StringOperationExpression::Concat {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            dot: 0,
        };

        assert_eq!(concat.to_string(), "$foo.$bar");
    }

    #[test]
    fn test_array_operation_expression_display() {
        let access = ArrayOperationExpression::Access {
            comments: CommentGroup { comments: vec![] },
            array: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            left_bracket: 0,
            index: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            right_bracket: 0,
        };

        assert_eq!(access.to_string(), "$foo[$bar]");

        let push = ArrayOperationExpression::Push {
            comments: CommentGroup { comments: vec![] },
            array: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            left_bracket: 0,
            right_bracket: 0,
        };

        assert_eq!(push.to_string(), "$foo[]");

        let unset = ArrayOperationExpression::Unset {
            comments: CommentGroup { comments: vec![] },
            unset: Keyword::new(ByteString::from("unset"), 0),
            item: Box::new(Expression::ArrayOperation(
                ArrayOperationExpression::Access {
                    comments: CommentGroup { comments: vec![] },
                    array: Box::new(Expression::Variable(Variable {
                        position: 0,
                        name: ByteString::from("foo"),
                    })),
                    left_bracket: 0,
                    index: Box::new(Expression::Variable(Variable {
                        position: 0,
                        name: ByteString::from("bar"),
                    })),
                    right_bracket: 0,
                },
            )),
        };

        assert_eq!(unset.to_string(), "unset $foo[$bar]");

        let isset = ArrayOperationExpression::Isset {
            comments: CommentGroup { comments: vec![] },
            isset: Keyword::new(ByteString::from("isset"), 0),
            item: Box::new(Expression::ArrayOperation(
                ArrayOperationExpression::Access {
                    comments: CommentGroup { comments: vec![] },
                    array: Box::new(Expression::Variable(Variable {
                        position: 0,
                        name: ByteString::from("foo"),
                    })),
                    left_bracket: 0,
                    index: Box::new(Expression::Variable(Variable {
                        position: 0,
                        name: ByteString::from("bar"),
                    })),
                    right_bracket: 0,
                },
            )),
        };

        assert_eq!(isset.to_string(), "isset $foo[$bar]");

        let r#in = ArrayOperationExpression::In {
            comments: CommentGroup { comments: vec![] },
            item: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("1"),
            }))),
            r#in: Keyword::new(ByteString::from("in"), 0),
            array: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(r#in.to_string(), "1 in $foo");
    }

    #[test]
    fn test_coalesce_operation_expression_display() {
        let coalesce = CoalesceOperationExpression::Coalesce {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            double_question: 0,
            right: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(coalesce.to_string(), "$foo ?? $bar");
    }

    #[test]
    fn test_ternary_operation_expression_display() {
        let ternary = TernaryOperationExpression::Ternary {
            comments: CommentGroup { comments: vec![] },
            condition: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            question: 0,
            if_true: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            colon: 0,
            if_false: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("baz"),
            })),
        };

        assert_eq!(ternary.to_string(), "$foo ? $bar : $baz");

        let short_ternary = TernaryOperationExpression::ShortTernary {
            comments: CommentGroup { comments: vec![] },
            condition: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            if_false: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
            question_colon: 0,
        };

        assert_eq!(short_ternary.to_string(), "$foo ?: $bar");

        let implicit_short_ternary = TernaryOperationExpression::ImplicitShortTernary {
            comments: CommentGroup { comments: vec![] },
            condition: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            question: 0,
            colon: 0,
            if_false: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(implicit_short_ternary.to_string(), "$foo ? : $bar");
    }

    #[test]
    fn test_type_operation_expression_display() {
        let instance_of = TypeOperationExpression::Instanceof {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            instanceof: Keyword::new(ByteString::from("instanceof"), 0),
            right: Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            },
        };

        assert_eq!(instance_of.to_string(), "$foo instanceof Foo");

        let is = TypeOperationExpression::Is {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            is: Keyword::new(ByteString::from("is"), 0),
            right: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(Keyword::new(
                ByteString::from("i64"),
                0,
            ))),
        };

        assert_eq!(is.to_string(), "$foo is i64");

        let into = TypeOperationExpression::Into {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            into: Keyword::new(ByteString::from("into"), 0),
            right: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(Keyword::new(
                ByteString::from("i64"),
                0,
            ))),
        };

        assert_eq!(into.to_string(), "$foo into i64");

        let r#as = TypeOperationExpression::As {
            comments: CommentGroup { comments: vec![] },
            left: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            r#as: Keyword::new(ByteString::from("as"), 0),
            right: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(Keyword::new(
                ByteString::from("i64"),
                0,
            ))),
        };

        assert_eq!(r#as.to_string(), "$foo as i64");
    }

    #[test]
    fn test_generator_operation_expression_display() {
        let r#yield = GeneratorOperationExpression::Yield {
            comments: CommentGroup { comments: vec![] },
            r#yield: Keyword::new(ByteString::from("yield"), 0),
        };

        assert_eq!(r#yield.to_string(), "yield");

        let yield_value = GeneratorOperationExpression::YieldValue {
            comments: CommentGroup { comments: vec![] },
            r#yield: Keyword::new(ByteString::from("yield"), 0),
            value: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(yield_value.to_string(), "yield $foo");

        let yield_key_value = GeneratorOperationExpression::YieldKeyValue {
            comments: CommentGroup { comments: vec![] },
            r#yield: Keyword::new(ByteString::from("yield"), 0),
            key: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            double_arrow: 0,
            value: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("bar"),
            })),
        };

        assert_eq!(yield_key_value.to_string(), "yield $foo => $bar");

        let yield_from = GeneratorOperationExpression::YieldFrom {
            comments: CommentGroup { comments: vec![] },
            r#yield: Keyword::new(ByteString::from("yield"), 0),
            from: Keyword::new(ByteString::from("from"), 0),
            value: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(yield_from.to_string(), "yield from $foo");
    }

    #[test]
    fn test_exception_operation_expression_display() {
        let throw = ExceptionOperationExpression::Throw {
            comments: CommentGroup { comments: vec![] },
            r#throw: Keyword::new(ByteString::from("throw"), 0),
            value: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(throw.to_string(), "throw $foo");
    }

    #[test]
    fn test_object_operation_expression_display() {
        let clone = ObjectOperationExpression::Clone {
            comments: CommentGroup { comments: vec![] },
            clone: Keyword::new(ByteString::from("clone"), 0),
            object: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(clone.to_string(), "clone $foo");

        let method_call = ObjectOperationExpression::MethodCall {
            comments: CommentGroup { comments: vec![] },
            object: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            arrow: 0,
            method: Identifier {
                position: 0,
                value: ByteString::from("bar"),
            },
            generics: None,
            arguments: ArgumentListExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                arguments: CommaSeparated {
                    inner: vec![ArgumentExpression::Value {
                        comments: CommentGroup { comments: vec![] },
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("1"),
                        })),
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
        };

        assert_eq!(method_call.to_string(), "$foo->bar(1)");

        let nullsafe_method_call = ObjectOperationExpression::NullsafeMethodCall {
            comments: CommentGroup { comments: vec![] },
            object: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            question_arrow: 0,
            method: Identifier {
                position: 0,
                value: ByteString::from("bar"),
            },
            generics: None,
            arguments: ArgumentListExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                arguments: CommaSeparated {
                    inner: vec![ArgumentExpression::Value {
                        comments: CommentGroup { comments: vec![] },
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("1"),
                        })),
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
        };

        assert_eq!(nullsafe_method_call.to_string(), "$foo?->bar(1)");

        let method_closure_creation = ObjectOperationExpression::MethodClosureCreation {
            comments: CommentGroup { comments: vec![] },
            object: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            arrow: 0,
            method: Identifier {
                position: 0,
                value: ByteString::from("bar"),
            },
            generics: None,
            placeholder: ArgumentPlaceholderExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                ellipsis: 0,
                right_parenthesis: 0,
            },
        };

        assert_eq!(method_closure_creation.to_string(), "$foo->bar(...)");

        let property_fetch = ObjectOperationExpression::PropertyFetch {
            comments: CommentGroup { comments: vec![] },
            object: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            arrow: 0,
            property: Identifier {
                position: 0,
                value: ByteString::from("bar"),
            },
        };

        assert_eq!(property_fetch.to_string(), "$foo->bar");

        let nullsafe_property_fetch = ObjectOperationExpression::NullsafePropertyFetch {
            comments: CommentGroup { comments: vec![] },
            object: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
            question_arrow: 0,
            property: Identifier {
                position: 0,
                value: ByteString::from("bar"),
            },
        };

        assert_eq!(nullsafe_property_fetch.to_string(), "$foo?->bar");
    }

    #[test]
    fn test_class_operation_initialization_class_expression_display() {
        let identifier = ClassOperationInitializationClassExpression::Identifier(Identifier {
            position: 0,
            value: ByteString::from("Foo"),
        });

        assert_eq!(identifier.to_string(), "new Foo");

        let variable = ClassOperationInitializationClassExpression::Variable(Variable {
            position: 0,
            name: ByteString::from("foo"),
        });

        assert_eq!(variable.to_string(), "new $foo");
    }

    #[test]
    fn test_class_operation_expression() {
        let initialization = ClassOperationExpression::Initialization {
            comments: CommentGroup { comments: vec![] },
            new: Keyword::new(ByteString::from("new"), 0),
            class: ClassOperationInitializationClassExpression::Identifier(Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            }),
            generics: None,
            arguments: ArgumentListExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                arguments: CommaSeparated {
                    inner: vec![ArgumentExpression::Value {
                        comments: CommentGroup { comments: vec![] },
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("1"),
                        })),
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
        };

        assert_eq!(initialization.to_string(), "new Foo(1)");

        let anonymous_initialization = ClassOperationExpression::AnonymousInitialization {
            comments: CommentGroup { comments: vec![] },
            new: Keyword::new(ByteString::from("new"), 0),
            class: AnonymousClassExpression {
                comments: CommentGroup { comments: vec![] },
                attributes: vec![],
                class: Keyword::new(ByteString::from("class"), 0),
                arguments: ArgumentListExpression {
                    comments: CommentGroup { comments: vec![] },
                    left_parenthesis: 0,
                    arguments: CommaSeparated {
                        inner: vec![],
                        commas: vec![],
                    },
                    right_parenthesis: 0,
                },
                extends: None,
                implements: None,
                body: ClassDefinitionBody {
                    left_brace: 0,
                    members: vec![],
                    right_brace: 0,
                },
            },
        };

        assert_eq!(
            anonymous_initialization.to_string(),
            "new class { /* ... */ }"
        );

        let anonymous_initialization_with_argument =
            ClassOperationExpression::AnonymousInitialization {
                comments: CommentGroup { comments: vec![] },
                new: Keyword::new(ByteString::from("new"), 0),
                class: AnonymousClassExpression {
                    comments: CommentGroup { comments: vec![] },
                    attributes: vec![],
                    class: Keyword::new(ByteString::from("class"), 0),
                    arguments: ArgumentListExpression {
                        comments: CommentGroup { comments: vec![] },
                        left_parenthesis: 0,
                        arguments: CommaSeparated {
                            inner: vec![ArgumentExpression::Value {
                                comments: CommentGroup { comments: vec![] },
                                value: Expression::Literal(Integer(LiteralInteger {
                                    comments: CommentGroup { comments: vec![] },
                                    position: 0,
                                    value: ByteString::from("1"),
                                })),
                            }],
                            commas: vec![],
                        },
                        right_parenthesis: 0,
                    },
                    extends: None,
                    implements: None,
                    body: ClassDefinitionBody {
                        left_brace: 0,
                        members: vec![],
                        right_brace: 0,
                    },
                },
            };

        assert_eq!(
            anonymous_initialization_with_argument.to_string(),
            "new class(1) { /* ... */ }"
        );

        let static_method_call = ClassOperationExpression::StaticMethodCall {
            comments: CommentGroup { comments: vec![] },
            class: Box::new(Expression::Identifier(Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            })),
            double_colon: 0,
            method: Identifier {
                position: 0,
                value: ByteString::from("bar"),
            },
            generics: None,
            arguments: ArgumentListExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                arguments: CommaSeparated {
                    inner: vec![ArgumentExpression::Value {
                        comments: CommentGroup { comments: vec![] },
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("1"),
                        })),
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
        };

        assert_eq!(static_method_call.to_string(), "Foo::bar(1)");

        let static_method_closure = ClassOperationExpression::StaticMethodClosureCreation {
            comments: CommentGroup { comments: vec![] },
            class: Box::new(Expression::Identifier(Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            })),
            double_colon: 0,
            method: Identifier {
                position: 0,
                value: ByteString::from("bar"),
            },
            generics: None,
            placeholder: ArgumentPlaceholderExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                ellipsis: 0,
                right_parenthesis: 0,
            },
        };

        assert_eq!(static_method_closure.to_string(), "Foo::bar(...)");

        let static_property_fetch = ClassOperationExpression::StaticPropertyFetch {
            comments: CommentGroup { comments: vec![] },
            class: Box::new(Expression::Identifier(Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            })),
            double_colon: 0,
            property: Variable {
                position: 0,
                name: ByteString::from("bar"),
            },
        };

        assert_eq!(static_property_fetch.to_string(), "Foo::$bar");

        let constant_fetch = ClassOperationExpression::ConstantFetch {
            comments: CommentGroup { comments: vec![] },
            class: Box::new(Expression::Identifier(Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            })),
            double_colon: 0,
            constant: Identifier {
                position: 0,
                value: ByteString::from("BAR"),
            },
        };

        assert_eq!(constant_fetch.to_string(), "Foo::BAR");
    }

    #[test]
    fn test_function_operation_expression() {
        let function_call = FunctionOperationExpression::Call {
            comments: CommentGroup { comments: vec![] },
            function: Box::new(Expression::Identifier(Identifier {
                position: 0,
                value: ByteString::from("foo"),
            })),
            generics: None,
            arguments: ArgumentListExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                arguments: CommaSeparated {
                    inner: vec![ArgumentExpression::Value {
                        comments: CommentGroup { comments: vec![] },
                        value: Expression::Literal(Integer(LiteralInteger {
                            comments: CommentGroup { comments: vec![] },
                            position: 0,
                            value: ByteString::from("1"),
                        })),
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
        };

        assert_eq!(function_call.to_string(), "foo(1)");

        let function_closure = FunctionOperationExpression::ClosureCreation {
            comments: CommentGroup { comments: vec![] },
            function: Box::new(Expression::Identifier(Identifier {
                position: 0,
                value: ByteString::from("foo"),
            })),
            generics: None,
            placeholder: ArgumentPlaceholderExpression {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                ellipsis: 0,
                right_parenthesis: 0,
            },
        };

        assert_eq!(function_closure.to_string(), "foo(...)");
    }

    #[test]
    fn test_async_operation_expression() {
        let r#async = AsyncOperationExpression::Async {
            comments: CommentGroup { comments: vec![] },
            r#async: Keyword::new(ByteString::from("async"), 0),
            expression: Box::new(Expression::FunctionOperation(
                FunctionOperationExpression::Call {
                    comments: CommentGroup { comments: vec![] },
                    function: Box::new(Expression::Identifier(Identifier {
                        position: 0,
                        value: ByteString::from("foo"),
                    })),
                    generics: None,
                    arguments: ArgumentListExpression {
                        comments: CommentGroup { comments: vec![] },
                        left_parenthesis: 0,
                        arguments: CommaSeparated {
                            inner: vec![ArgumentExpression::Value {
                                comments: CommentGroup { comments: vec![] },
                                value: Expression::Literal(Integer(LiteralInteger {
                                    comments: CommentGroup { comments: vec![] },
                                    position: 0,
                                    value: ByteString::from("1"),
                                })),
                            }],
                            commas: vec![],
                        },
                        right_parenthesis: 0,
                    },
                },
            )),
        };

        assert_eq!(r#async.to_string(), "async foo(1)");

        let r#await = AsyncOperationExpression::Await {
            comments: CommentGroup { comments: vec![] },
            r#await: Keyword::new(ByteString::from("await"), 0),
            expression: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(r#await.to_string(), "await $foo");

        let concurrently = AsyncOperationExpression::Concurrently {
            comments: CommentGroup { comments: vec![] },
            concurrently: Keyword::new(ByteString::from("concurrently"), 0),
            left_brace: 0,
            expressions: CommaSeparated {
                inner: vec![],
                commas: vec![],
            },
            right_brace: 0,
        };

        assert_eq!(concurrently.to_string(), "concurrently {{ /* ... */ }}");
    }

    #[test]
    fn test_range_operation_expression() {
        let between = RangeOperationExpression::Between {
            comments: CommentGroup { comments: vec![] },
            from: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("1"),
            }))),
            double_dot: 0,
            to: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("10"),
            }))),
        };

        assert_eq!(between.to_string(), "1..10");

        let between_inclusive = RangeOperationExpression::BetweenInclusive {
            comments: CommentGroup { comments: vec![] },
            from: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("1"),
            }))),
            double_dot: 0,
            to: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("10"),
            }))),
            equals: 0,
        };

        assert_eq!(between_inclusive.to_string(), "1..=10");

        let to = RangeOperationExpression::To {
            comments: CommentGroup { comments: vec![] },
            double_dot: 0,
            to: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("10"),
            }))),
        };

        assert_eq!(to.to_string(), "..10");

        let to_inclusive = RangeOperationExpression::ToInclusive {
            comments: CommentGroup { comments: vec![] },
            double_dot: 0,
            to: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("10"),
            }))),
            equals: 0,
        };

        assert_eq!(to_inclusive.to_string(), "..=10");

        let from = RangeOperationExpression::From {
            comments: CommentGroup { comments: vec![] },
            from: Box::new(Expression::Literal(Integer(LiteralInteger {
                comments: CommentGroup { comments: vec![] },
                position: 0,
                value: ByteString::from("10"),
            }))),
            double_dot: 0,
        };

        assert_eq!(from.to_string(), "10..");

        let full = RangeOperationExpression::Full {
            comments: CommentGroup { comments: vec![] },
            double_dot: 0,
        };

        assert_eq!(full.to_string(), "..");
    }
}
