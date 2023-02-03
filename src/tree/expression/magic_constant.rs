use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::byte_string::ByteString;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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
        match &self {
            Self::Directory { position, .. } => *position,
            Self::File { position, .. } => *position,
            Self::Line { position, .. } => *position,
            Self::Class { position, .. } => *position,
            Self::Function { position, .. } => *position,
            Self::Method { position, .. } => *position,
            Self::Namespace { position, .. } => *position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Directory { position, value } => position + value.len(),
            Self::File { position, value } => position + value.len(),
            Self::Line { position, value } => position + value.len(),
            Self::Class { position, value } => position + value.len(),
            Self::Function { position, value } => position + value.len(),
            Self::Method { position, value } => position + value.len(),
            Self::Namespace { position, value } => position + value.len(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Directory { .. } => "directory magic constant expression".to_string(),
            Self::File { .. } => "file magic constant expression".to_string(),
            Self::Line { .. } => "line magic constant expression".to_string(),
            Self::Class { .. } => "class magic constant expression".to_string(),
            Self::Function { .. } => "function magic constant expression".to_string(),
            Self::Method { .. } => "method magic constant expression".to_string(),
            Self::Namespace { .. } => "namespace magic constant expression".to_string(),
        }
    }
}
