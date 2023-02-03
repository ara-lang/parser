use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::statement::Statement;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BlockStatement {
    pub comments: CommentGroup,
    pub left_brace: usize,
    pub statements: Vec<Statement>,
    pub right_brace: usize,
}

impl Node for BlockStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.left_brace
    }

    fn final_position(&self) -> usize {
        self.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.statements
            .iter()
            .map(|statement| statement as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "block statement".to_string()
    }
}
