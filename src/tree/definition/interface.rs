use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeDefinitionGroup;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::function::AbstractConstructorDefinition;
use crate::tree::definition::function::AbstractMethodDefinition;
use crate::tree::definition::template::TemplateGroupDefinition;
use crate::tree::identifier::Identifier;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InterfaceDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeDefinitionGroup>,
    pub interface: Span,
    pub name: Identifier,
    pub templates: Option<TemplateGroupDefinition>,
    pub extends: Option<InterfaceDefinitionExtends>,
    pub body: InterfaceDefinitionBody,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InterfaceDefinitionExtends {
    pub extends: Span,
    pub parents: CommaSeparated<TemplatedIdentifier>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InterfaceDefinitionBody {
    pub left_brace: Span,
    pub members: Vec<InterfaceDefinitionMember>,
    pub right_brace: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum InterfaceDefinitionMember {
    Constant(ClassishConstantDefinition),
    Constructor(AbstractConstructorDefinition),
    Method(AbstractMethodDefinition),
}

impl Node for InterfaceDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            attributes.initial_position()
        } else {
            self.interface.position
        }
    }

    fn final_position(&self) -> usize {
        self.body.right_brace.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.name];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(templates) = &self.templates {
            children.push(templates);
        }

        if let Some(extends) = &self.extends {
            children.push(extends);
        }

        children.push(&self.body);

        children
    }
}

impl Node for InterfaceDefinitionExtends {
    fn initial_position(&self) -> usize {
        self.extends.position
    }

    fn final_position(&self) -> usize {
        if let Some(last_interface) = self.parents.inner.last() {
            let last_interface_position = last_interface.final_position();
            if let Some(last_comma) = self.parents.commas.last() {
                let last_comma_position = last_comma.position + 1;
                if last_comma_position > last_interface_position {
                    return last_comma_position;
                }
            }

            return last_interface_position;
        }

        self.extends.position + 7
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.parents
            .inner
            .iter()
            .map(|item| item as &dyn Node)
            .collect()
    }
}

impl Node for InterfaceDefinitionBody {
    fn initial_position(&self) -> usize {
        self.left_brace.position
    }

    fn final_position(&self) -> usize {
        self.right_brace.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members.iter().map(|item| item as &dyn Node).collect()
    }
}

impl Node for InterfaceDefinitionMember {
    fn initial_position(&self) -> usize {
        match self {
            Self::Constant(constant) => constant.initial_position(),
            Self::Constructor(constructor) => constructor.initial_position(),
            Self::Method(method) => method.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            Self::Constant(constant) => constant.final_position(),
            Self::Constructor(constructor) => constructor.final_position(),
            Self::Method(method) => method.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Self::Constant(constant) => vec![constant],
            Self::Constructor(constructor) => vec![constructor],
            Self::Method(method) => vec![method],
        }
    }
}
