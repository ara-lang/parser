use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::definition::Definition;
use crate::tree::identifier::Identifier;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NamespaceDefinition {
    pub namespace: Span,
    pub name: Identifier,
    pub semicolon: Span,
    pub definitions: Vec<Definition>,
}

impl Node for NamespaceDefinition {
    fn initial_position(&self) -> usize {
        self.namespace.position
    }

    fn final_position(&self) -> usize {
        self.semicolon.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.name];

        for definition in &self.definitions {
            children.push(definition);
        }

        children
    }
}
