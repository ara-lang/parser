use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::modifier::ModifierGroupDefinition;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PropertyDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    pub modifiers: ModifierGroupDefinition,
    pub type_definition: TypeDefinition,
    pub entry: PropertyEntryDefinition,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum PropertyEntryDefinition {
    Uninitialized {
        variable: Variable,
    },
    Initialized {
        variable: Variable,
        equals: usize,
        value: Expression,
    },
}

impl PropertyEntryDefinition {
    pub fn variable(&self) -> &Variable {
        match &self {
            Self::Uninitialized { variable } => variable,
            Self::Initialized { variable, .. } => variable,
        }
    }
}

impl Node for PropertyDefinition {
    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        if let Some(modifier) = self.modifiers.modifiers.first() {
            return modifier.initial_position();
        }

        self.type_definition.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        children.push(&self.modifiers);
        children.push(&self.type_definition);
        children.push(&self.entry);

        children
    }

    fn get_description(&self) -> String {
        "property definition".to_string()
    }
}

impl Node for PropertyEntryDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Uninitialized { variable } => variable.initial_position(),
            Self::Initialized { variable, .. } => variable.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Uninitialized { variable } => variable.final_position(),
            Self::Initialized { value, .. } => value.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Uninitialized { variable } => vec![variable],
            Self::Initialized {
                variable, value, ..
            } => vec![variable, value],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Uninitialized { .. } => "uninitialized property entry".to_string(),
            Self::Initialized { .. } => "initialized property entry".to_string(),
        }
    }
}

impl std::fmt::Display for PropertyDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {};",
            self.modifiers, self.type_definition, self.entry
        )
    }
}

impl std::fmt::Display for PropertyEntryDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Uninitialized { variable } => write!(f, "{}", variable),
            Self::Initialized {
                variable, value, ..
            } => write!(f, "{} = {}", variable, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::comment::CommentGroup;
    use crate::tree::definition::modifier::ModifierDefinition;
    use crate::tree::definition::r#type::UnsignedIntegerTypeDefinition;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;
    use crate::tree::token::Keyword;

    #[test]
    pub fn test_property_definition_display() {
        let uninitialized_property_definition = PropertyDefinition {
            attributes: vec![],
            modifiers: ModifierGroupDefinition {
                position: 0,
                modifiers: vec![
                    ModifierDefinition::Public(Keyword::new(ByteString::from("public"), 0)),
                    ModifierDefinition::Readonly(Keyword::new(ByteString::from("readonly"), 0)),
                ],
            },
            type_definition: TypeDefinition::UnsignedInteger(UnsignedIntegerTypeDefinition::U32(
                Keyword::new(ByteString::from("u32"), 15),
            )),
            entry: PropertyEntryDefinition::Uninitialized {
                variable: Variable {
                    position: 0,
                    name: ByteString::from("foo"),
                },
            },
            semicolon: 0,
        };

        assert_eq!(
            uninitialized_property_definition.to_string(),
            "public readonly u32 $foo;"
        );

        let initialized_property_definition = PropertyDefinition {
            attributes: vec![],
            modifiers: ModifierGroupDefinition {
                position: 0,
                modifiers: vec![
                    ModifierDefinition::Public(Keyword::new(ByteString::from("public"), 0)),
                    ModifierDefinition::Readonly(Keyword::new(ByteString::from("readonly"), 0)),
                ],
            },
            type_definition: TypeDefinition::UnsignedInteger(UnsignedIntegerTypeDefinition::U8(
                Keyword::new(ByteString::from("u8"), 7),
            )),
            entry: PropertyEntryDefinition::Initialized {
                variable: Variable {
                    position: 0,
                    name: ByteString::from("bar"),
                },
                equals: 0,
                value: Expression::Literal(Integer(LiteralInteger {
                    comments: CommentGroup { comments: vec![] },
                    position: 0,
                    value: ByteString::from("123"),
                })),
            },
            semicolon: 0,
        };

        assert_eq!(
            initialized_property_definition.to_string(),
            "public readonly u8 $bar = 123;"
        );
    }
}
