use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::Definition;
use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NamespaceDefinition {
    pub namespace: Keyword,
    pub name: Identifier,
    pub semicolon: usize,
    pub definitions: Vec<Definition>,
}

impl Node for NamespaceDefinition {
    fn initial_position(&self) -> usize {
        self.namespace.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.namespace, &self.name];

        for definition in &self.definitions {
            children.push(definition);
        }

        children
    }

    fn get_description(&self) -> String {
        "namespace definition".to_string()
    }
}

impl std::fmt::Display for NamespaceDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {};", self.namespace, self.name)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;

    #[test]
    fn test_namespace_definition_display() {
        let namespace_definition = NamespaceDefinition {
            namespace: Keyword {
                value: ByteString::from("namespace"),
                position: 0,
            },
            name: Identifier {
                position: 0,
                value: ByteString::from("Foo\\Bar"),
            },
            semicolon: 14,
            definitions: vec![],
        };

        assert_eq!(namespace_definition.to_string(), "namespace Foo\\Bar;");
    }
}
