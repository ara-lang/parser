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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DefinitionTree {
    pub definitions: Vec<Definition>,
    pub eof: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
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
}

impl Node for Definition {
    fn initial_position(&self) -> usize {
        match self {
            Definition::Namespace(definition) => definition.initial_position(),
            Definition::Use(definition) => definition.initial_position(),
            Definition::TypeAlias(definition) => definition.initial_position(),
            Definition::Constant(definition) => definition.initial_position(),
            Definition::Function(definition) => definition.initial_position(),
            Definition::Interface(definition) => definition.initial_position(),
            Definition::Enum(definition) => definition.initial_position(),
            Definition::Class(definition) => definition.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            Definition::Namespace(definition) => definition.final_position(),
            Definition::Use(definition) => definition.final_position(),
            Definition::TypeAlias(definition) => definition.final_position(),
            Definition::Constant(definition) => definition.final_position(),
            Definition::Function(definition) => definition.final_position(),
            Definition::Interface(definition) => definition.final_position(),
            Definition::Enum(definition) => definition.final_position(),
            Definition::Class(definition) => definition.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Definition::Namespace(definition) => vec![definition.as_ref()],
            Definition::Use(definition) => vec![definition.as_ref()],
            Definition::TypeAlias(definition) => vec![definition.as_ref()],
            Definition::Constant(definition) => vec![definition.as_ref()],
            Definition::Function(definition) => vec![definition.as_ref()],
            Definition::Interface(definition) => vec![definition.as_ref()],
            Definition::Enum(definition) => vec![definition.as_ref()],
            Definition::Class(definition) => vec![definition.as_ref()],
        }
    }
}
