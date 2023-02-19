use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::expression::argument::ArgumentListExpression;
use crate::tree::identifier::Identifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AttributeGroupDefinition {
    pub hash_left_bracket: usize,
    pub members: CommaSeparated<AttributeDefinition>,
    pub right_bracket: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AttributeDefinition {
    pub name: Identifier,
    pub arguments: Option<ArgumentListExpression>,
}

impl Node for AttributeGroupDefinition {
    fn initial_position(&self) -> usize {
        self.hash_left_bracket
    }

    fn final_position(&self) -> usize {
        self.right_bracket + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .inner
            .iter()
            .map(|member| member as &dyn Node)
            .collect()
    }

    fn get_description(&self) -> String {
        "attribute group definition".to_string()
    }
}

impl Node for AttributeDefinition {
    fn initial_position(&self) -> usize {
        self.name.initial_position()
    }

    fn final_position(&self) -> usize {
        if let Some(arguments) = &self.arguments {
            arguments.final_position()
        } else {
            self.name.final_position()
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        if let Some(arguments) = &self.arguments {
            vec![&self.name, arguments]
        } else {
            vec![&self.name]
        }
    }

    fn get_description(&self) -> String {
        "attribute definition".to_string()
    }
}

impl std::fmt::Display for AttributeGroupDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[{}]", self.members)
    }
}

impl std::fmt::Display for AttributeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(arguments) = &self.arguments {
            write!(f, "{}", arguments)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::comment::CommentGroup;
    use crate::tree::expression::argument::ArgumentExpression;
    use crate::tree::expression::literal::Literal::Integer;
    use crate::tree::expression::literal::LiteralInteger;
    use crate::tree::expression::Expression;

    #[test]
    fn test_attribute_group_definition_display() {
        let attribute_group_definition = AttributeGroupDefinition {
            hash_left_bracket: 0,
            members: CommaSeparated {
                inner: vec![
                    AttributeDefinition {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Foo"),
                        },
                        arguments: None,
                    },
                    AttributeDefinition {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Bar"),
                        },
                        arguments: Some(ArgumentListExpression {
                            comments: CommentGroup { comments: vec![] },
                            left_parenthesis: 24,
                            arguments: CommaSeparated {
                                inner: vec![ArgumentExpression::Value {
                                    comments: CommentGroup { comments: vec![] },
                                    value: Expression::Literal(Integer(LiteralInteger {
                                        comments: CommentGroup { comments: vec![] },
                                        position: 0,
                                        value: ByteString::from("2"),
                                    })),
                                }],
                                commas: vec![],
                            },
                            right_parenthesis: 30,
                        }),
                    },
                ],
                commas: vec![],
            },
            right_bracket: 32,
        };

        assert_eq!(attribute_group_definition.to_string(), "#[Foo, Bar(2)]");
    }
}
