use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UseDefinitionSymbolAlias {
    pub r#as: Keyword,
    pub alias: Identifier,
}

impl Node for UseDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Default { r#use, .. }
            | Self::Function { r#use, .. }
            | Self::Constant { r#use, .. } => r#use.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Default { semicolon, .. }
            | Self::Function { semicolon, .. }
            | Self::Constant { semicolon, .. } => semicolon + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Default {
                r#use, name, alias, ..
            } => {
                let mut children: Vec<&dyn Node> = vec![r#use, name];

                if let Some(alias) = alias {
                    children.push(alias);
                }

                children
            }
            Self::Function {
                r#use,
                function: r#type,
                name,
                alias,
                ..
            }
            | Self::Constant {
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
            Self::Default { .. } => "use definition".to_string(),
            Self::Function { .. } => "use function definition".to_string(),
            Self::Constant { .. } => "use constant definition".to_string(),
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
