use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ArgumentExpression {
    Value {
        comments: CommentGroup,
        value: Expression,
    },
    Spread {
        comments: CommentGroup,
        ellipsis: usize,
        value: Expression,
    },
    ReverseSpread {
        comments: CommentGroup,
        value: Expression,
        ellipsis: usize,
    },
    Named {
        comments: CommentGroup,
        name: Identifier,
        colon: usize,
        value: Expression,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArgumentListExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub arguments: CommaSeparated<ArgumentExpression>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArgumentPlaceholderExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: usize,
    pub ellipsis: usize,
    pub right_parenthesis: usize,
}

impl Node for ArgumentExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ArgumentExpression::Value { comments, .. }
            | ArgumentExpression::Spread { comments, .. }
            | ArgumentExpression::ReverseSpread { comments, .. }
            | ArgumentExpression::Named { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ArgumentExpression::Value { value, .. }
            | ArgumentExpression::ReverseSpread { value, .. } => value.initial_position(),
            ArgumentExpression::Spread { ellipsis, .. } => *ellipsis,
            ArgumentExpression::Named { name, .. } => name.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ArgumentExpression::Value { value, .. } | ArgumentExpression::Spread { value, .. } => {
                value.final_position()
            }
            ArgumentExpression::ReverseSpread { ellipsis, .. } => *ellipsis,
            ArgumentExpression::Named { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ArgumentExpression::Value { value, .. }
            | ArgumentExpression::Spread { value, .. }
            | ArgumentExpression::ReverseSpread { value, .. } => vec![value],
            ArgumentExpression::Named { name, value, .. } => vec![name, value],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            ArgumentExpression::Value { .. } => "value argument expression".to_string(),
            ArgumentExpression::Spread { .. } => "spread argument expression".to_string(),
            ArgumentExpression::ReverseSpread { .. } => {
                "reverse spread argument expression".to_string()
            }
            ArgumentExpression::Named { .. } => "named argument expression".to_string(),
        }
    }
}

impl Node for ArgumentListExpression {
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
        self.arguments
            .inner
            .iter()
            .map(|a| a as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "argument list expression".to_string()
    }
}

impl Node for ArgumentPlaceholderExpression {
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
        vec![]
    }

    fn get_description(&self) -> String {
        "argument placeholder expression".to_string()
    }
}
