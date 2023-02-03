use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::expression::argument::ArgumentListExpression;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AttributeGroupDefinition {
    pub hash_left_bracket: usize,
    pub members: CommaSeparated<AttributeDefinition>,
    pub right_bracket: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AttributeDefinition {
    pub name: Identifier,
    pub arguments: Option<ArgumentListExpression>,
}

impl Node for AttributeGroupDefinition {
    fn initial_position(&self) -> usize {
        self.hash_left_bracket
    }

    fn final_position(&self) -> usize {
        self.right_bracket + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .inner
            .iter()
            .map(|member| member as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "attribute group definition".to_string()
    }
}

impl Node for AttributeDefinition {
    fn initial_position(&self) -> usize {
        self.name.initial_position()
    }

    fn final_position(&self) -> usize {
        if let Some(arguments) = &self.arguments {
            arguments.final_position()
        } else {
            self.name.final_position()
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        if let Some(arguments) = &self.arguments {
            vec![&self.name, arguments]
        } else {
            vec![&self.name]
        }
    }

    fn get_description(&self) -> String {
        "attribute definition".to_string()
    }
}
