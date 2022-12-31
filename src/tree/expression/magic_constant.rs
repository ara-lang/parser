use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::lexer::token::Span;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum MagicConstant {
    Directory { span: Span, value: ByteString },
    File { span: Span, value: ByteString },
    Line { span: Span, value: ByteString },
    Class { span: Span, value: ByteString },
    Function { span: Span, value: ByteString },
    Method { span: Span, value: ByteString },
    Namespace { span: Span, value: ByteString },
}

impl Node for MagicConstant {
    fn initial_position(&self) -> usize {
        match self {
            MagicConstant::Directory { span, .. } => span.position,
            MagicConstant::File { span, .. } => span.position,
            MagicConstant::Line { span, .. } => span.position,
            MagicConstant::Class { span, .. } => span.position,
            MagicConstant::Function { span, .. } => span.position,
            MagicConstant::Method { span, .. } => span.position,
            MagicConstant::Namespace { span, .. } => span.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            MagicConstant::Directory { span, value } => span.position + value.len(),
            MagicConstant::File { span, value } => span.position + value.len(),
            MagicConstant::Line { span, value } => span.position + value.len(),
            MagicConstant::Class { span, value } => span.position + value.len(),
            MagicConstant::Function { span, value } => span.position + value.len(),
            MagicConstant::Method { span, value } => span.position + value.len(),
            MagicConstant::Namespace { span, value } => span.position + value.len(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
