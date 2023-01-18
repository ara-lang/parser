use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum UseDefinition {
    Default {
        r#use: Keyword,
        name: Identifier,
        alias: Option<UseDefinitionSymbolAlias>,
        semicolon: usize,
    },
    // use function a as b;
    Function {
        r#use: Keyword,
        function: Keyword,
        name: Identifier,
        alias: Option<UseDefinitionSymbolAlias>,
        semicolon: usize,
    },
    Constant {
        r#use: Keyword,
        r#const: Keyword,
        name: Identifier,
        alias: Option<UseDefinitionSymbolAlias>,
        semicolon: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UseDefinitionSymbolAlias {
    pub r#as: Keyword,
    pub alias: Identifier,
}

impl Node for UseDefinition {
    fn initial_position(&self) -> usize {
        match self {
            UseDefinition::Default { r#use, .. }
            | UseDefinition::Function { r#use, .. }
            | UseDefinition::Constant { r#use, .. } => r#use.initial_position(),
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
            UseDefinition::Default {
                r#use, name, alias, ..
            } => {
                let mut children: Vec<&dyn Node> = vec![r#use, name];

                if let Some(alias) = alias {
                    children.push(alias);
                }

                children
            }
            UseDefinition::Function {
                r#use,
                function: r#type,
                name,
                alias,
                ..
            }
            | UseDefinition::Constant {
                r#use,
                r#const: r#type,
                name,
                alias,
                ..
            } => {
                let mut children: Vec<&dyn Node> = vec![r#use, r#type, name];

                if let Some(alias) = alias {
                    children.push(alias);
                }

                children
            }
        }
    }

    fn get_description(&self) -> String {
        match &self {
            UseDefinition::Default { .. } => "use definition".to_string(),
            UseDefinition::Function { .. } => "use function definition".to_string(),
            UseDefinition::Constant { .. } => "use constant definition".to_string(),
        }
    }
}

impl Node for UseDefinitionSymbolAlias {
    fn initial_position(&self) -> usize {
        self.r#as.initial_position()
    }

    fn final_position(&self) -> usize {
        self.alias.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.r#as, &self.alias]
    }

    fn get_description(&self) -> String {
        "use symbol alias definition".to_string()
    }
}
