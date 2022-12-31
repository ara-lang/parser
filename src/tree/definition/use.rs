use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::identifier::Identifier;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum UseDefinition {
    Default {
        r#use: usize,
        name: Identifier,
        alias: Option<UseDefinitionSymbolAlias>,
        semicolon: usize,
    },
    // use function a as b;
    Function {
        r#use: usize,
        function: usize,
        name: Identifier,
        alias: Option<UseDefinitionSymbolAlias>,
        semicolon: usize,
    },
    Constant {
        r#use: usize,
        r#const: usize,
        name: Identifier,
        alias: Option<UseDefinitionSymbolAlias>,
        semicolon: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UseDefinitionSymbolAlias {
    pub r#as: usize,
    pub alias: Identifier,
}

impl Node for UseDefinition {
    fn initial_position(&self) -> usize {
        match self {
            UseDefinition::Default { r#use, .. }
            | UseDefinition::Function { r#use, .. }
            | UseDefinition::Constant { r#use, .. } => *r#use,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            UseDefinition::Default { semicolon, .. }
            | UseDefinition::Function { semicolon, .. }
            | UseDefinition::Constant { semicolon, .. } => semicolon + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            UseDefinition::Default { name, alias, .. }
            | UseDefinition::Function { name, alias, .. }
            | UseDefinition::Constant { name, alias, .. } => {
                let mut children: Vec<&dyn Node> = vec![name];

                if let Some(alias) = alias {
                    children.push(alias);
                }

                children
            }
        }
    }
}

impl Node for UseDefinitionSymbolAlias {
    fn initial_position(&self) -> usize {
        self.r#as
    }

    fn final_position(&self) -> usize {
        self.alias.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.alias]
    }
}
