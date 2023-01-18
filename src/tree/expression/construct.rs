use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ExitConstructExpression {
    Exit {
        comments: CommentGroup,
        exit: Keyword,
    },
    // `exit(42)`
    ExitWith {
        comments: CommentGroup,
        exit: Keyword,
        left_parenthesis: usize,
        value: Option<Box<Expression>>,
        right_parenthesis: usize,
    },
}

impl Node for ExitConstructExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            ExitConstructExpression::Exit { comments, .. } => Some(comments),
            ExitConstructExpression::ExitWith { comments, .. } => Some(comments),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            ExitConstructExpression::Exit { exit, .. }
            | ExitConstructExpression::ExitWith { exit, .. } => exit.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            ExitConstructExpression::Exit { exit, .. } => exit.final_position(),
            ExitConstructExpression::ExitWith {
                right_parenthesis, ..
            } => right_parenthesis + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            ExitConstructExpression::Exit { exit, .. } => vec![exit],
            ExitConstructExpression::ExitWith { exit, value, .. } => {
                if let Some(value) = value {
                    vec![exit, value.as_ref()]
                } else {
                    vec![exit]
                }
            }
        }
    }

    fn get_description(&self) -> String {
        "exit construct expression".to_string()
    }
}
