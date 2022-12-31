use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExpressionStatement {
    pub comments: CommentGroup,
    pub expression: Expression,
    pub semicolon: Span,
}

impl Node for ExpressionStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.expression.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.expression]
    }
}
