use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ArgumentExpression {
    Positional {
        comments: CommentGroup,
        ellipsis: Option<Span>,
        value: Expression,
    },
    Named {
        comments: CommentGroup,
        name: Identifier,
        colon: Span,
        ellipsis: Option<Span>,
        value: Expression,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArgumentListExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,
    pub arguments: CommaSeparated<ArgumentExpression>,
    pub right_parenthesis: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArgumentPlaceholderExpression {
    pub comments: CommentGroup,
    pub left_parenthesis: Span,
    pub ellipsis: Span,
    pub right_parenthesis: Span,
}

impl Node for ArgumentExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ArgumentExpression::Positional { comments, .. }
            | ArgumentExpression::Named { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match self {
            ArgumentExpression::Positional {
                ellipsis, value, ..
            } => ellipsis
                .map(|ellipsis| ellipsis.position)
                .unwrap_or_else(|| value.initial_position()),
            ArgumentExpression::Named { name, .. } => name.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ArgumentExpression::Positional { value, .. } => value.final_position(),
            ArgumentExpression::Named { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            ArgumentExpression::Positional { value, .. } => vec![value],
            ArgumentExpression::Named { name, value, .. } => vec![name, value],
        }
    }
}

impl Node for ArgumentListExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.left_parenthesis.position
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.arguments
            .inner
            .iter()
            .map(|a| a as &dyn Node)
            .collect()
    }
}

impl Node for ArgumentPlaceholderExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.left_parenthesis.position
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
