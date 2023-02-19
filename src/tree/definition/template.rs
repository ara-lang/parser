use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::r#type::TypeDefinition;
use crate::tree::identifier::Identifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemplateDefinitionVariance {
    Covariance(usize),
    Invaraint,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemplateDefinitionTypeConstraint {
    SubType(Keyword, TypeDefinition),
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TemplateDefinition {
    pub variance: TemplateDefinitionVariance,
    pub name: Identifier,
    pub constraint: TemplateDefinitionTypeConstraint,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TemplateGroupDefinition {
    pub comments: CommentGroup,
    pub less_than: usize,
    pub members: CommaSeparated<TemplateDefinition>,
    pub greater_than: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
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

    fn get_description(&self) -> String {
        "template definition".to_string()
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

    fn get_description(&self) -> String {
        "template group definition".to_string()
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

    fn get_description(&self) -> String {
        "type template group definition".to_string()
    }
}

impl std::fmt::Display for TemplateGroupDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>",
            self.members
                .inner
                .iter()
                .map(|type_definition| { type_definition.to_string() })
                .collect::<Vec<String>>()
                .join(", ")
        )
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
                .map(|type_definition| { type_definition.to_string() })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl std::fmt::Display for TemplateDefinitionVariance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Covariance(_) => write!(f, "+"),
            Self::Invaraint => write!(f, ""),
        }
    }
}

impl std::fmt::Display for TemplateDefinitionTypeConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SubType(k, t) => write!(f, " {k} {t}"),
            Self::None => write!(f, ""),
        }
    }
}

impl std::fmt::Display for TemplateDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "{}{}{}",
            &self.variance, self.name, &self.constraint
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::identifier::TemplatedIdentifier;

    #[test]
    fn test_template_definition_display() {
        let template_definition = TemplateDefinition {
            variance: TemplateDefinitionVariance::Covariance(0),
            name: Identifier {
                position: 1,
                value: ByteString::from("T"),
            },
            constraint: TemplateDefinitionTypeConstraint::SubType(
                Keyword {
                    value: ByteString::from("as"),
                    position: 2,
                },
                TypeDefinition::Identifier(TemplatedIdentifier {
                    name: Identifier {
                        position: 3,
                        value: ByteString::from("object"),
                    },
                    templates: None,
                }),
            ),
        };

        assert_eq!(template_definition.to_string(), "+T as object");

        let template_definition = TemplateDefinition {
            variance: TemplateDefinitionVariance::Invaraint,
            name: Identifier {
                position: 1,
                value: ByteString::from("U"),
            },
            constraint: TemplateDefinitionTypeConstraint::SubType(
                Keyword {
                    value: ByteString::from("as"),
                    position: 2,
                },
                TypeDefinition::Identifier(TemplatedIdentifier {
                    name: Identifier {
                        position: 3,
                        value: ByteString::from("IFoo"),
                    },
                    templates: None,
                }),
            ),
        };

        assert_eq!(template_definition.to_string(), "U as IFoo");
    }

    #[test]
    fn test_template_group_definition_display() {
        let template_group_definition = TemplateGroupDefinition {
            comments: CommentGroup { comments: vec![] },
            less_than: 0,
            members: CommaSeparated {
                inner: vec![
                    TemplateDefinition {
                        variance: TemplateDefinitionVariance::Covariance(0),
                        name: Identifier {
                            position: 1,
                            value: ByteString::from("T"),
                        },
                        constraint: TemplateDefinitionTypeConstraint::SubType(
                            Keyword {
                                value: ByteString::from("as"),
                                position: 2,
                            },
                            TypeDefinition::Identifier(TemplatedIdentifier {
                                name: Identifier {
                                    position: 3,
                                    value: ByteString::from("object"),
                                },
                                templates: None,
                            }),
                        ),
                    },
                    TemplateDefinition {
                        variance: TemplateDefinitionVariance::Invaraint,
                        name: Identifier {
                            position: 1,
                            value: ByteString::from("U"),
                        },
                        constraint: TemplateDefinitionTypeConstraint::SubType(
                            Keyword {
                                value: ByteString::from("as"),
                                position: 2,
                            },
                            TypeDefinition::Identifier(TemplatedIdentifier {
                                name: Identifier {
                                    position: 3,
                                    value: ByteString::from("IFoo"),
                                },
                                templates: None,
                            }),
                        ),
                    },
                ],
                commas: vec![],
            },
            greater_than: 4,
        };

        assert_eq!(
            template_group_definition.to_string(),
            "<+T as object, U as IFoo>"
        );

        let template_group_definition = TemplateGroupDefinition {
            comments: CommentGroup { comments: vec![] },
            less_than: 0,
            members: CommaSeparated {
                inner: vec![
                    TemplateDefinition {
                        variance: TemplateDefinitionVariance::Invaraint,
                        name: Identifier {
                            position: 1,
                            value: ByteString::from("T"),
                        },
                        constraint: TemplateDefinitionTypeConstraint::None,
                    },
                    TemplateDefinition {
                        variance: TemplateDefinitionVariance::Invaraint,
                        name: Identifier {
                            position: 1,
                            value: ByteString::from("U"),
                        },
                        constraint: TemplateDefinitionTypeConstraint::None,
                    },
                ],
                commas: vec![],
            },
            greater_than: 4,
        };

        assert_eq!(template_group_definition.to_string(), "<T, U>");
    }
}
