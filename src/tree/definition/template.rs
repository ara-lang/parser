use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemplateDefinitionVariance {
    Covariance(Span),
    Invaraint,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemplateDefinitionTypeConstraint {
    SubType(Span, TypeDefinition),
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TemplateDefinition {
    pub variance: TemplateDefinitionVariance,
    pub name: Identifier,
    pub constraint: TemplateDefinitionTypeConstraint,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TemplateGroupDefinition {
    pub comments: CommentGroup,
    pub less_than: Span,
    pub members: CommaSeparated<TemplateDefinition>,
    pub greater_than: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeTemplateGroupDefinition {
    pub comments: CommentGroup,
    pub less_than: Span,
    pub members: CommaSeparated<TypeDefinition>,
    pub greater_than: Span,
}

impl Node for TemplateDefinition {
    fn initial_position(&self) -> usize {
        match &self.variance {
            TemplateDefinitionVariance::Covariance(s) => s.position,
            TemplateDefinitionVariance::Invaraint => self.name.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self.constraint {
            TemplateDefinitionTypeConstraint::SubType(_, t) => t.final_position(),
            TemplateDefinitionTypeConstraint::None => self.name.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.name];

        match &self.constraint {
            TemplateDefinitionTypeConstraint::SubType(_, t) => {
                children.push(t);

                children
            }
            TemplateDefinitionTypeConstraint::None => children,
        }
    }
}

impl Node for TemplateGroupDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.less_than.position
    }

    fn final_position(&self) -> usize {
        self.greater_than.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members.inner.iter().map(|s| s as &dyn Node).collect()
    }
}

impl Node for TypeTemplateGroupDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.less_than.position
    }

    fn final_position(&self) -> usize {
        self.greater_than.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members.inner.iter().map(|s| s as &dyn Node).collect()
    }
}

impl std::fmt::Display for TypeTemplateGroupDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>",
            self.members
                .inner
                .iter()
                .map(|s| { s.to_string() })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
