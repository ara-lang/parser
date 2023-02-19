use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use std::fmt::Display;

use crate::lexer::byte_string::ByteString;
use crate::tree::comment::CommentGroup;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Keyword {
    pub value: ByteString,
    pub position: usize,
}

impl Keyword {
    pub fn new(value: ByteString, position: usize) -> Self {
        Self { value, position }
    }
}

impl Node for Keyword {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        self.position
    }

    fn final_position(&self) -> usize {
        self.position + self.value.len()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }

    fn get_description(&self) -> String {
        "keyword".to_string()
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_display() {
        let keyword = Keyword::new(ByteString::from("iterable"), 0);

        assert_eq!(keyword.to_string(), "iterable");
    }
}
