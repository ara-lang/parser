use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ModifierDefinition {
    Public(Keyword),
    Protected(Keyword),
    Private(Keyword),
    Static(Keyword),
    Readonly(Keyword),
    Final(Keyword),
    Abstract(Keyword),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroupDefinition {
    pub position: usize,
    pub modifiers: Vec<ModifierDefinition>,
}

impl Node for ModifierGroupDefinition {
    fn initial_position(&self) -> usize {
        self.modifiers
            .first()
            .map(|modifier| modifier.initial_position())
            .unwrap_or(self.position)
    }

    fn final_position(&self) -> usize {
        self.modifiers
            .last()
            .map(|modifier| modifier.final_position())
            .unwrap_or(self.position)
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.modifiers
            .iter()
            .map(|modifier| modifier as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "modifier group definition".to_string()
    }
}

impl Node for ModifierDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Public(keyword)
            | Self::Protected(keyword)
            | Self::Private(keyword)
            | Self::Readonly(keyword)
            | Self::Static(keyword)
            | Self::Abstract(keyword)
            | Self::Final(keyword) => keyword.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Public(keyword)
            | Self::Protected(keyword)
            | Self::Private(keyword)
            | Self::Readonly(keyword)
            | Self::Static(keyword)
            | Self::Abstract(keyword)
            | Self::Final(keyword) => keyword.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Public(keyword)
            | Self::Protected(keyword)
            | Self::Private(keyword)
            | Self::Readonly(keyword)
            | Self::Static(keyword)
            | Self::Abstract(keyword)
            | Self::Final(keyword) => vec![keyword as &dyn Node],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Public(_keyword) => "public modifier definition".to_string(),
            Self::Protected(_keyword) => "protected modifier definition".to_string(),
            Self::Private(_keyword) => "private modifier definition".to_string(),
            Self::Readonly(_keyword) => "readonly modifier definition".to_string(),
            Self::Static(_keyword) => "static modifier definition".to_string(),
            Self::Abstract(_keyword) => "abstract modifier definition".to_string(),
            Self::Final(_keyword) => "final modifier definition".to_string(),
        }
    }
}

impl std::fmt::Display for ModifierDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Public(keyword)
            | Self::Protected(keyword)
            | Self::Private(keyword)
            | Self::Readonly(keyword)
            | Self::Static(keyword)
            | Self::Abstract(keyword)
            | Self::Final(keyword) => write!(f, "{}", keyword.value),
        }
    }
}
