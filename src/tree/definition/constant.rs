use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::modifier::ConstantModifierDefinitionGroup;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstantDefinitionEntry {
    pub name: Identifier,
    pub equals: usize,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstantDefinition {
    pub comments: CommentGroup,
    pub r#const: Keyword,
    pub entries: CommaSeparated<ConstantDefinitionEntry>,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassishConstantDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    #[serde(flatten)]
    pub modifiers: ConstantModifierDefinitionGroup,
    pub r#const: Keyword,
    pub entries: CommaSeparated<ConstantDefinitionEntry>,
    pub semicolon: usize,
}

impl Node for ConstantDefinitionEntry {
    fn initial_position(&self) -> usize {
        self.name.initial_position()
    }

    fn final_position(&self) -> usize {
        self.value.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.name, &self.value]
    }
}

impl Node for ConstantDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#const.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#const];

        for entry in &self.entries.inner {
            children.push(entry);
        }

        children
    }
}

impl Node for ClassishConstantDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            attribute.initial_position()
        } else if let Some(modifier) = self.modifiers.modifiers.first() {
            modifier.initial_position()
        } else {
            self.r#const.initial_position()
        }
    }

    fn final_position(&self) -> usize {
        self.semicolon
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];
        for attribute in &self.attributes {
            children.push(attribute);
        }

        for modifier in &self.modifiers.modifiers {
            children.push(modifier);
        }

        children.push(&self.r#const);

        for entry in &self.entries.inner {
            children.push(entry);
        }

        children
    }
}
