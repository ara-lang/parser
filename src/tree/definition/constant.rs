use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::modifier::ModifierGroupDefinition;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstantDefinition {
    pub comments: CommentGroup,
    pub r#const: Keyword,
    pub type_definition: TypeDefinition,
    pub name: Identifier,
    pub equals: usize,
    pub value: Expression,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassishConstantDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub modifiers: ModifierGroupDefinition,
    pub r#const: Keyword,
    pub type_definition: TypeDefinition,
    pub name: Identifier,
    pub equals: usize,
    pub value: Expression,
    pub semicolon: usize,
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
        vec![
            &self.r#const,
            &self.type_definition,
            &self.name,
            &self.value,
        ]
    }

    fn get_description(&self) -> String {
        "constant definition".to_string()
    }
}

impl Node for ClassishConstantDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        self.modifiers.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];
        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.modifiers);
        children.push(&self.r#const);
        children.push(&self.type_definition);
        children.push(&self.name);
        children.push(&self.value);

        children
    }

    fn get_description(&self) -> String {
        "classish constant definition".to_string()
    }
}
