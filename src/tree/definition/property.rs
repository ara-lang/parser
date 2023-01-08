use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::modifier::PropertyModifierDefinitionGroup;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PropertyDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    #[serde(flatten)]
    pub modifiers: PropertyModifierDefinitionGroup,
    pub type_definition: TypeDefinition,
    pub entry: PropertyEntryDefinition,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum PropertyEntryDefinition {
    Uninitialized {
        variable: Variable,
    },
    Initialized {
        variable: Variable,
        equals: usize,
        value: Expression,
    },
}

impl PropertyEntryDefinition {
    pub fn variable(&self) -> &Variable {
        match self {
            PropertyEntryDefinition::Uninitialized { variable } => variable,
            PropertyEntryDefinition::Initialized { variable, .. } => variable,
        }
    }
}

impl Node for PropertyDefinition {
    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        if let Some(modifier) = self.modifiers.modifiers.first() {
            return modifier.initial_position();
        }

        self.type_definition.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        for modifier in &self.modifiers.modifiers {
            children.push(modifier);
        }

        children.push(&self.type_definition);
        children.push(&self.entry);

        children
    }
}

impl Node for PropertyEntryDefinition {
    fn initial_position(&self) -> usize {
        match self {
            PropertyEntryDefinition::Uninitialized { variable } => variable.initial_position(),
            PropertyEntryDefinition::Initialized { variable, .. } => variable.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            PropertyEntryDefinition::Uninitialized { variable } => variable.final_position(),
            PropertyEntryDefinition::Initialized { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            PropertyEntryDefinition::Uninitialized { variable } => vec![variable],
            PropertyEntryDefinition::Initialized {
                variable, value, ..
            } => vec![variable, value],
        }
    }
}
