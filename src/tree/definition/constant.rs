use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeDefinitionGroup;
use crate::tree::definition::modifier::ConstantModifierDefinitionGroup;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstantDefinitionEntry {
    pub name: Identifier,
    pub equals: Span,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConstantDefinition {
    pub comments: CommentGroup,
    pub r#const: Span,
    pub entries: CommaSeparated<ConstantDefinitionEntry>,
    pub semicolon: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassishConstantDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeDefinitionGroup>,
    #[serde(flatten)]
    pub modifiers: ConstantModifierDefinitionGroup,
    pub r#const: Span,
    pub entries: CommaSeparated<ConstantDefinitionEntry>,
    pub semicolon: Span,
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
        self.r#const.position
    }

    fn final_position(&self) -> usize {
        self.semicolon.position
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.entries
            .inner
            .iter()
            .map(|entry| entry as &dyn Node)
            .collect()
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
            self.r#const.position
        }
    }

    fn final_position(&self) -> usize {
        self.semicolon.position
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children = self
            .attributes
            .iter()
            .map(|attribute| attribute as &dyn Node)
            .collect::<Vec<&dyn Node>>();

        children.extend(
            self.modifiers
                .modifiers
                .iter()
                .map(|modifier| modifier as &dyn Node),
        );

        children.extend(self.entries.inner.iter().map(|entry| entry as &dyn Node));

        children
    }
}
