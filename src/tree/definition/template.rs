use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemplateDefinitionVariance {
    Covariance(usize),
    Invaraint,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemplateDefinitionTypeConstraint {
    SubType(Keyword, TypeDefinition),
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
    pub less_than: usize,
    pub members: CommaSeparated<TemplateDefinition>,
    pub greater_than: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeTemplateGroupDefinition {
    pub comments: CommentGroup,
    pub less_than: usize,
    pub members: CommaSeparated<TypeDefinition>,
    pub greater_than: usize,
}

impl Node for TemplateDefinition {
    fn initial_position(&self) -> usize {
        match &self.variance {
            TemplateDefinitionVariance::Covariance(position) => *position,
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
        match &self.constraint {
            TemplateDefinitionTypeConstraint::SubType(k, t) => {
                vec![&self.name, k, t]
            }
            TemplateDefinitionTypeConstraint::None => vec![&self.name],
        }
    }
}

impl Node for TemplateGroupDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.less_than
    }

    fn final_position(&self) -> usize {
        self.greater_than + 1
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
        self.less_than
    }

    fn final_position(&self) -> usize {
        self.greater_than + 1
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
