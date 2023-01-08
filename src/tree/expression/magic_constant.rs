use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum MagicConstant {
    Directory { position: usize, value: ByteString },
    File { position: usize, value: ByteString },
    Line { position: usize, value: ByteString },
    Class { position: usize, value: ByteString },
    Function { position: usize, value: ByteString },
    Method { position: usize, value: ByteString },
    Namespace { position: usize, value: ByteString },
}

impl Node for MagicConstant {
    fn initial_position(&self) -> usize {
        match self {
            MagicConstant::Directory { position, .. } => *position,
            MagicConstant::File { position, .. } => *position,
            MagicConstant::Line { position, .. } => *position,
            MagicConstant::Class { position, .. } => *position,
            MagicConstant::Function { position, .. } => *position,
            MagicConstant::Method { position, .. } => *position,
            MagicConstant::Namespace { position, .. } => *position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            MagicConstant::Directory { position, value } => position + value.len(),
            MagicConstant::File { position, value } => position + value.len(),
            MagicConstant::Line { position, value } => position + value.len(),
            MagicConstant::Class { position, value } => position + value.len(),
            MagicConstant::Function { position, value } => position + value.len(),
            MagicConstant::Method { position, value } => position + value.len(),
            MagicConstant::Namespace { position, value } => position + value.len(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
