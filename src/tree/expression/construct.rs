use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ExitConstructExpression {
    Exit {
        comments: CommentGroup,
        exit: Span,
    },
    // `exit(42)`
    ExitWith {
        comments: CommentGroup,
        exit: Span,
        left_parenthesis: Span,
        value: Option<Box<Expression>>,
        right_parenthesis: Span,
    },
}

impl Node for ExitConstructExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match self {
            ExitConstructExpression::Exit { comments, .. } => Some(comments),
            ExitConstructExpression::ExitWith { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match self {
            ExitConstructExpression::Exit { exit, .. } => exit.position,
            ExitConstructExpression::ExitWith { exit, .. } => exit.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ExitConstructExpression::Exit { exit, .. } => exit.position + 4,
            ExitConstructExpression::ExitWith {
                right_parenthesis, ..
            } => right_parenthesis.position + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            ExitConstructExpression::Exit { .. } => vec![],
            ExitConstructExpression::ExitWith { value, .. } => {
                if let Some(value) = value {
                    vec![value.as_ref()]
                } else {
                    vec![]
                }
            }
        }
    }
}
