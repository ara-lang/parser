use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum Literal {
    String(LiteralString),
    Integer(LiteralInteger),
    Float(LiteralFloat),
    Null(LiteralNull),
    True(LiteralTrue),
    False(LiteralFalse),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralString {
    pub comments: CommentGroup,
    pub value: ByteString,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralInteger {
    pub comments: CommentGroup,
    pub value: ByteString,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralFloat {
    pub comments: CommentGroup,
    pub value: ByteString,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralNull {
    pub comments: CommentGroup,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralTrue {
    pub comments: CommentGroup,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LiteralFalse {
    pub comments: CommentGroup,
    pub span: Span,
}

impl Node for Literal {
    fn comments(&self) -> Option<&CommentGroup> {
        match self {
            Literal::String(literal) => literal.comments(),
            Literal::Integer(literal) => literal.comments(),
            Literal::Float(literal) => literal.comments(),
            Literal::Null(literal) => literal.comments(),
            Literal::True(literal) => literal.comments(),
            Literal::False(literal) => literal.comments(),
        }
    }

    fn initial_position(&self) -> usize {
        match self {
            Literal::String(literal) => literal.initial_position(),
            Literal::Integer(literal) => literal.initial_position(),
            Literal::Float(literal) => literal.initial_position(),
            Literal::Null(literal) => literal.initial_position(),
            Literal::True(literal) => literal.initial_position(),
            Literal::False(literal) => literal.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
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
}

impl Node for LiteralString {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.span.position
    }

    fn final_position(&self) -> usize {
        self.span.position + self.value.len()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for LiteralInteger {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.span.position
    }

    fn final_position(&self) -> usize {
        self.span.position + self.value.len()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for LiteralFloat {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.span.position
    }

    fn final_position(&self) -> usize {
        self.span.position + self.value.len()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for LiteralNull {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.span.position
    }

    fn final_position(&self) -> usize {
        self.span.position + 4
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for LiteralTrue {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.span.position
    }

    fn final_position(&self) -> usize {
        self.span.position + 4
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for LiteralFalse {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.span.position
    }

    fn final_position(&self) -> usize {
        self.span.position + 5
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
