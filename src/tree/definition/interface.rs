use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::function::AbstractConstructorDefinition;
use crate::tree::definition::function::AbstractMethodDefinition;
use crate::tree::definition::template::TemplateGroupDefinition;
use crate::tree::identifier::Identifier;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InterfaceDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub interface: Keyword,
    pub name: Identifier,
    pub templates: Option<TemplateGroupDefinition>,
    pub extends: Option<InterfaceDefinitionExtends>,
    pub body: InterfaceDefinitionBody,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InterfaceDefinitionExtends {
    pub extends: Keyword,
    pub parents: CommaSeparated<TemplatedIdentifier>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InterfaceDefinitionBody {
    pub left_brace: usize,
    pub members: Vec<InterfaceDefinitionMember>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
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
            self.interface.initial_position()
        }
    }

    fn final_position(&self) -> usize {
        self.body.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.interface);
        children.push(&self.name);

        if let Some(templates) = &self.templates {
            children.push(templates);
        }

        if let Some(extends) = &self.extends {
            children.push(extends);
        }

        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "interface definition".to_string()
    }
}

impl Node for InterfaceDefinitionExtends {
    fn initial_position(&self) -> usize {
        self.extends.initial_position()
    }

    fn final_position(&self) -> usize {
        if let Some(last_interface) = self.parents.inner.last() {
            let last_interface_position = last_interface.final_position();
            if let Some(last_comma) = self.parents.commas.last() {
                let last_comma_position = last_comma + 1;
                if last_comma_position > last_interface_position {
                    return last_comma_position;
                }
            }

            return last_interface_position;
        }

        self.extends.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.extends];
        for parent in &self.parents.inner {
            children.push(parent);
        }

        children
    }

    fn get_description(&self) -> String {
        "interface extends definition".to_string()
    }
}

impl Node for InterfaceDefinitionBody {
    fn initial_position(&self) -> usize {
        self.left_brace
    }

    fn final_position(&self) -> usize {
        self.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members.iter().map(|item| item as &dyn Node).collect()
    }

    fn get_description(&self) -> String {
        "interface body definition".to_string()
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

    fn get_description(&self) -> String {
        "interface member definition".to_string()
    }
}
