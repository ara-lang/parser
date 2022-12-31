use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ExitConstructExpression {
    Exit {
        comments: CommentGroup,
        exit: usize,
    },
    // `exit(42)`
    ExitWith {
        comments: CommentGroup,
        exit: usize,
        left_parenthesis: usize,
        value: Option<Box<Expression>>,
        right_parenthesis: usize,
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
            ExitConstructExpression::Exit { exit, .. } => *exit,
            ExitConstructExpression::ExitWith { exit, .. } => *exit,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ExitConstructExpression::Exit { exit, .. } => exit + 4,
            ExitConstructExpression::ExitWith {
                right_parenthesis, ..
            } => right_parenthesis + 1,
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
