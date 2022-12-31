use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::statement::Statement;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BlockStatement {
    pub comments: CommentGroup,
    pub left_brace: Span,
    pub statements: Vec<Statement>,
    pub right_brace: Span,
}

impl Node for BlockStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.left_brace.position
    }

    fn final_position(&self) -> usize {
        self.right_brace.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.statements
            .iter()
            .map(|statement| statement as &dyn Node)
            .collect()
    }
}
