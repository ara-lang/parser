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
