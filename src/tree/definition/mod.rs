use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::class::ClassDefinition;
use crate::tree::definition::constant::ConstantDefinition;
use crate::tree::definition::function::FunctionDefinition;
use crate::tree::definition::interface::InterfaceDefinition;
use crate::tree::definition::namespace::NamespaceDefinition;
use crate::tree::definition::r#enum::EnumDefinition;
use crate::tree::definition::r#type::TypeAliasDefinition;
use crate::tree::definition::r#use::UseDefinition;
use crate::tree::Node;

pub mod attribute;
pub mod class;
pub mod constant;
pub mod r#enum;
pub mod function;
pub mod interface;
pub mod modifier;
pub mod namespace;
pub mod property;
pub mod template;
pub mod r#type;
pub mod r#use;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DefinitionTree {
    pub definitions: Vec<Definition>,
    pub eof: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum Definition {
    Namespace(Box<NamespaceDefinition>),
    Use(Box<UseDefinition>),
    TypeAlias(Box<TypeAliasDefinition>),
    Constant(Box<ConstantDefinition>),
    Function(Box<FunctionDefinition>),
    Interface(Box<InterfaceDefinition>),
    Enum(Box<EnumDefinition>),
    Class(Box<ClassDefinition>),
}

impl Node for DefinitionTree {
    fn initial_position(&self) -> usize {
        0
    }

    fn final_position(&self) -> usize {
        self.eof
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.definitions.iter().map(|d| d as &dyn Node).collect()
    }

    fn get_description(&self) -> String {
        "tree definition".to_string()
    }
}

impl Node for Definition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Namespace(definition) => definition.initial_position(),
            Self::Use(definition) => definition.initial_position(),
            Self::TypeAlias(definition) => definition.initial_position(),
            Self::Constant(definition) => definition.initial_position(),
            Self::Function(definition) => definition.initial_position(),
            Self::Interface(definition) => definition.initial_position(),
            Self::Enum(definition) => definition.initial_position(),
            Self::Class(definition) => definition.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Namespace(definition) => definition.final_position(),
            Self::Use(definition) => definition.final_position(),
            Self::TypeAlias(definition) => definition.final_position(),
            Self::Constant(definition) => definition.final_position(),
            Self::Function(definition) => definition.final_position(),
            Self::Interface(definition) => definition.final_position(),
            Self::Enum(definition) => definition.final_position(),
            Self::Class(definition) => definition.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Namespace(definition) => vec![definition.as_ref()],
            Self::Use(definition) => vec![definition.as_ref()],
            Self::TypeAlias(definition) => vec![definition.as_ref()],
            Self::Constant(definition) => vec![definition.as_ref()],
            Self::Function(definition) => vec![definition.as_ref()],
            Self::Interface(definition) => vec![definition.as_ref()],
            Self::Enum(definition) => vec![definition.as_ref()],
            Self::Class(definition) => vec![definition.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Namespace(definition) => definition.get_description(),
            Self::Use(definition) => definition.get_description(),
            Self::TypeAlias(definition) => definition.get_description(),
            Self::Constant(definition) => definition.get_description(),
            Self::Function(definition) => definition.get_description(),
            Self::Interface(definition) => definition.get_description(),
            Self::Enum(definition) => definition.get_description(),
            Self::Class(definition) => definition.get_description(),
        }
    }
}
