use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::tree::comment::CommentGroup;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum Literal {
    String(LiteralString),
    Integer(LiteralInteger),
    Float(LiteralFloat),
    Null(LiteralNull),
    True(LiteralTrue),
    False(LiteralFalse),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralString {
    pub comments: CommentGroup,
    pub value: ByteString,
    pub position: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralInteger {
    pub comments: CommentGroup,
    pub value: ByteString,
    pub position: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralFloat {
    pub comments: CommentGroup,
    pub value: ByteString,
    pub position: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralNull {
    pub comments: CommentGroup,
    pub null: Keyword,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralTrue {
    pub comments: CommentGroup,
    pub r#true: Keyword,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralFalse {
    pub comments: CommentGroup,
    pub r#false: Keyword,
}

impl Node for Literal {
    fn comments(&self) -> Option<&CommentGroup> {
        match &self {
            Literal::String(literal) => literal.comments(),
            Literal::Integer(literal) => literal.comments(),
            Literal::Float(literal) => literal.comments(),
            Literal::Null(literal) => literal.comments(),
            Literal::True(literal) => literal.comments(),
            Literal::False(literal) => literal.comments(),
        }
    }

    fn initial_position(&self) -> usize {
        match &self {
            Literal::String(literal) => literal.initial_position(),
            Literal::Integer(literal) => literal.initial_position(),
            Literal::Float(literal) => literal.initial_position(),
            Literal::Null(literal) => literal.initial_position(),
            Literal::True(literal) => literal.initial_position(),
            Literal::False(literal) => literal.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Literal::String(literal) => literal.final_position(),
            Literal::Integer(literal) => literal.final_position(),
            Literal::Float(literal) => literal.final_position(),
            Literal::Null(literal) => literal.final_position(),
            Literal::True(literal) => literal.final_position(),
            Literal::False(literal) => literal.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Literal::String(literal) => vec![literal],
            Literal::Integer(literal) => vec![literal],
            Literal::Float(literal) => vec![literal],
            Literal::Null(literal) => vec![literal],
            Literal::True(literal) => vec![literal],
            Literal::False(literal) => vec![literal],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Literal::String(literal) => literal.get_description(),
            Literal::Integer(literal) => literal.get_description(),
            Literal::Float(literal) => literal.get_description(),
            Literal::Null(literal) => literal.get_description(),
            Literal::True(literal) => literal.get_description(),
            Literal::False(literal) => literal.get_description(),
        }
    }
}

impl Node for LiteralString {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
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
        "literal string expression".to_string()
    }
}

impl Node for LiteralInteger {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
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
        "literal integer expression".to_string()
    }
}

impl Node for LiteralFloat {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
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
        "literal float expression".to_string()
    }
}

impl Node for LiteralNull {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.null.initial_position()
    }

    fn final_position(&self) -> usize {
        self.null.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.null]
    }

    fn get_description(&self) -> String {
        "literal null expression".to_string()
    }
}

impl Node for LiteralTrue {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#true.initial_position()
    }

    fn final_position(&self) -> usize {
        self.r#true.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.r#true]
    }

    fn get_description(&self) -> String {
        "literal true expression".to_string()
    }
}

impl Node for LiteralFalse {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#false.initial_position()
    }

    fn final_position(&self) -> usize {
        self.r#false.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.r#false]
    }

    fn get_description(&self) -> String {
        "literal false expression".to_string()
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null(inner) => write!(f, "{}", inner),
            Self::False(inner) => write!(f, "{}", inner),
            Self::True(inner) => write!(f, "{}", inner),
            Self::Integer(inner) => write!(f, "{}", inner),
            Self::Float(inner) => write!(f, "{}", inner),
            Self::String(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::fmt::Display for LiteralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for LiteralInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for LiteralFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for LiteralNull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl std::fmt::Display for LiteralTrue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "true")
    }
}

impl std::fmt::Display for LiteralFalse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "false")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_display() {
        let string = Literal::String(LiteralString {
            comments: CommentGroup { comments: vec![] },
            value: ByteString::from("\"foo\""),
            position: 0,
        });

        assert_eq!(string.to_string(), "\"foo\"");

        let integer = LiteralInteger {
            comments: CommentGroup { comments: vec![] },
            position: 0,
            value: ByteString::from("123"),
        };

        assert_eq!(integer.to_string(), "123");

        let float = LiteralFloat {
            comments: CommentGroup { comments: vec![] },
            position: 0,
            value: ByteString::from("3.14"),
        };

        assert_eq!(float.to_string(), "3.14");

        let null = LiteralNull {
            comments: CommentGroup { comments: vec![] },
            null: Keyword::new(ByteString::from("null"), 0),
        };

        assert_eq!(null.to_string(), "null");

        let r#true = LiteralTrue {
            comments: CommentGroup { comments: vec![] },
            r#true: Keyword::new(ByteString::from("true"), 0),
        };

        assert_eq!(r#true.to_string(), "true");

        let r#false = LiteralFalse {
            comments: CommentGroup { comments: vec![] },
            r#false: Keyword::new(ByteString::from("false"), 0),
        };

        assert_eq!(r#false.to_string(), "false");
    }
}
