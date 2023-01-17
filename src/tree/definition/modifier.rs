use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
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
        match self {
            ModifierDefinition::Public(keyword)
            | ModifierDefinition::Protected(keyword)
            | ModifierDefinition::Private(keyword)
            | ModifierDefinition::Readonly(keyword)
            | ModifierDefinition::Static(keyword)
            | ModifierDefinition::Abstract(keyword)
            | ModifierDefinition::Final(keyword) => keyword.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ModifierDefinition::Public(keyword)
            | ModifierDefinition::Protected(keyword)
            | ModifierDefinition::Private(keyword)
            | ModifierDefinition::Readonly(keyword)
            | ModifierDefinition::Static(keyword)
            | ModifierDefinition::Abstract(keyword)
            | ModifierDefinition::Final(keyword) => keyword.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            ModifierDefinition::Public(keyword)
            | ModifierDefinition::Protected(keyword)
            | ModifierDefinition::Private(keyword)
            | ModifierDefinition::Readonly(keyword)
            | ModifierDefinition::Static(keyword)
            | ModifierDefinition::Abstract(keyword)
            | ModifierDefinition::Final(keyword) => vec![keyword as &dyn Node],
        }
    }

    fn get_description(&self) -> String {
        "modifier definition".to_string()
    }
}

impl std::fmt::Display for ModifierDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifierDefinition::Public(keyword)
            | ModifierDefinition::Protected(keyword)
            | ModifierDefinition::Private(keyword)
            | ModifierDefinition::Readonly(keyword)
            | ModifierDefinition::Static(keyword)
            | ModifierDefinition::Abstract(keyword)
            | ModifierDefinition::Final(keyword) => write!(f, "{}", keyword.value),
        }
    }
}
