use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ReturnStatement {
    Explicit {
        comments: CommentGroup,
        r#return: usize,
        expression: Option<Expression>,
        semicolon: usize,
    },
    Implicit {
        comments: CommentGroup,
        expression: Expression,
    },
}

impl Node for ReturnStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ReturnStatement::Explicit { comments, .. } => Some(comments),
            ReturnStatement::Implicit { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ReturnStatement::Explicit { r#return, .. } => *r#return,
            ReturnStatement::Implicit { expression, .. } => expression.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ReturnStatement::Explicit { semicolon, .. } => semicolon + 1,
            ReturnStatement::Implicit { expression, .. } => expression.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ReturnStatement::Explicit { expression, .. } => {
                if let Some(expression) = expression {
                    vec![expression]
                } else {
                    vec![]
                }
            }
            ReturnStatement::Implicit { expression, .. } => vec![expression],
        }
    }
}
