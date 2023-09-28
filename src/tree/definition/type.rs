use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::expression::literal::Literal;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeAliasDefinition {
    pub r#type: Keyword,
    pub name: TemplatedIdentifier,
    pub equals: usize,
    pub type_definition: TypeDefinition,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum SignedIntegerTypeDefinition {
    Default(Keyword), // 'int'
    I128(Keyword),    // 'i128'
    I64(Keyword),     // 'i64'
    I32(Keyword),     // 'i32'
    I16(Keyword),     // 'i16'
    I8(Keyword),      // 'i8'
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum UnsignedIntegerTypeDefinition {
    Default(Keyword), // 'uint'
    U32(Keyword),     // 'u32'
    U16(Keyword),     // 'u16'
    U8(Keyword),      // 'u8'
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum FloatingPointTypeDefinition {
    Default(Keyword), // 'float'
    F64(Keyword),     // 'f64'
    F32(Keyword),     // 'f32'
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TypeDefinition {
    Identifier(TemplatedIdentifier),
    Nullable(usize, Box<TypeDefinition>),
    Union(Vec<TypeDefinition>),
    Intersection(Vec<TypeDefinition>),
    Void(Keyword),
    Never(Keyword),
    Boolean(Keyword),
    String(Keyword),
    SignedInteger(SignedIntegerTypeDefinition),
    UnsignedInteger(UnsignedIntegerTypeDefinition),
    FloatingPoint(FloatingPointTypeDefinition),
    Dict(Keyword, TypeTemplateGroupDefinition),
    Vec(Keyword, TypeTemplateGroupDefinition),
    Object(Keyword),
    Mixed(Keyword),
    NonNull(Keyword),
    Resource(Keyword),
    Iterable(Keyword, TypeTemplateGroupDefinition),
    Class(Keyword, TypeTemplateGroupDefinition),
    Interface(Keyword, TypeTemplateGroupDefinition),
    Literal(Literal),
    Tuple {
        left_parenthesis: usize,
        type_definitions: CommaSeparated<TypeDefinition>,
        right_parenthesis: usize,
    },
    Parenthesized {
        left_parenthesis: usize,
        type_definition: Box<TypeDefinition>,
        right_parenthesis: usize,
    },
}

impl TypeDefinition {
    pub fn is_standalone(&self) -> bool {
        matches!(
            self,
            Self::Mixed(_)
                | Self::Never(_)
                | Self::Void(_)
                | Self::Nullable(_, _)
                | Self::NonNull(_)
                | Self::Resource(_)
        )
    }

    pub fn is_nullable(&self) -> bool {
        matches!(self, Self::Nullable(_, _))
    }

    pub fn is_bottom(&self) -> bool {
        matches!(self, Self::Never(_) | Self::Void(_))
    }

    pub fn is_scalar(&self, include_union: bool) -> bool {
        match &self {
            Self::Literal(literal) => !matches!(literal, Literal::Null(_)),
            | Self::Boolean(_)
            | Self::SignedInteger(_)
            | Self::UnsignedInteger(_)
            | Self::FloatingPoint(_)
            | Self::String(_)
            // class, and interface are represented as strings at runtime, so they are considered scalars
            | Self::Class(_, _)
            | Self::Interface(_, _) => true,
            Self::Union(definitions) if include_union => {
                definitions.iter().all(|definition| definition.is_scalar(true))
            },
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }
}

impl Node for TypeAliasDefinition {
    fn initial_position(&self) -> usize {
        self.r#type.initial_position()
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.r#type, &self.name, &self.type_definition]
    }

    fn get_description(&self) -> String {
        "type alias definition".to_string()
    }
}

impl Node for SignedIntegerTypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Default(keyword)
            | Self::I128(keyword)
            | Self::I64(keyword)
            | Self::I32(keyword)
            | Self::I16(keyword)
            | Self::I8(keyword) => keyword.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Default(keyword)
            | Self::I128(keyword)
            | Self::I64(keyword)
            | Self::I32(keyword)
            | Self::I16(keyword)
            | Self::I8(keyword) => keyword.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Default(keyword)
            | Self::I128(keyword)
            | Self::I64(keyword)
            | Self::I32(keyword)
            | Self::I16(keyword)
            | Self::I8(keyword) => vec![keyword],
        }
    }

    fn get_description(&self) -> String {
        "signed integer type definition".to_string()
    }
}

impl Node for UnsignedIntegerTypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Default(keyword)
            | Self::U32(keyword)
            | Self::U16(keyword)
            | Self::U8(keyword) => keyword.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Default(keyword)
            | Self::U32(keyword)
            | Self::U16(keyword)
            | Self::U8(keyword) => keyword.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Default(keyword)
            | Self::U32(keyword)
            | Self::U16(keyword)
            | Self::U8(keyword) => vec![keyword],
        }
    }

    fn get_description(&self) -> String {
        "unsigned integer type definition".to_string()
    }
}

impl Node for FloatingPointTypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Default(keyword) | Self::F64(keyword) | Self::F32(keyword) => {
                keyword.initial_position()
            }
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Default(keyword) | Self::F64(keyword) | Self::F32(keyword) => {
                keyword.final_position()
            }
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Default(keyword) | Self::F64(keyword) | Self::F32(keyword) => vec![keyword],
        }
    }

    fn get_description(&self) -> String {
        "floating point type definition".to_string()
    }
}

impl Node for TypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Identifier(inner) => inner.initial_position(),
            Self::Union(inner) => inner[0].initial_position(),
            Self::Intersection(inner) => inner[0].initial_position(),
            Self::Literal(literal) => literal.initial_position(),
            Self::Nullable(position, _) => *position,
            Self::Void(keyword)
            | Self::Never(keyword)
            | Self::Boolean(keyword)
            | Self::String(keyword)
            | Self::Dict(keyword, _)
            | Self::Vec(keyword, _)
            | Self::Object(keyword)
            | Self::Mixed(keyword)
            | Self::NonNull(keyword)
            | Self::Resource(keyword)
            | Self::Class(keyword, _)
            | Self::Interface(keyword, _)
            | Self::Iterable(keyword, _) => keyword.initial_position(),
            Self::SignedInteger(signed) => signed.initial_position(),
            Self::UnsignedInteger(unsigned) => unsigned.initial_position(),
            Self::FloatingPoint(floating) => floating.initial_position(),
            Self::Tuple {
                left_parenthesis: position,
                ..
            }
            | Self::Parenthesized {
                left_parenthesis: position,
                ..
            } => *position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Identifier(inner) => inner.final_position(),
            Self::Nullable(_, inner) => inner.final_position(),
            Self::Dict(_, template)
            | Self::Vec(_, template)
            | Self::Class(_, template)
            | Self::Interface(_, template)
            | Self::Iterable(_, template) => template.final_position(),
            Self::Union(inner) => inner[inner.len() - 1].final_position(),
            Self::Intersection(inner) => inner[inner.len() - 1].final_position(),
            Self::Literal(literal) => literal.final_position(),
            Self::Void(keyword)
            | Self::Never(keyword)
            | Self::Boolean(keyword)
            | Self::String(keyword)
            | Self::Object(keyword)
            | Self::Mixed(keyword)
            | Self::NonNull(keyword)
            | Self::Resource(keyword) => keyword.final_position(),
            Self::SignedInteger(signed) => signed.final_position(),
            Self::UnsignedInteger(unsigned) => unsigned.final_position(),
            Self::FloatingPoint(floating) => floating.final_position(),
            Self::Parenthesized {
                right_parenthesis, ..
            }
            | Self::Tuple {
                right_parenthesis, ..
            } => right_parenthesis + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Identifier(inner) => vec![inner],
            Self::Nullable(_, inner) => vec![inner.as_ref()],
            Self::Union(inner) | Self::Intersection(inner) => {
                inner.iter().map(|t| t as &dyn Node).collect()
            }
            Self::Void(keyword)
            | Self::Object(keyword)
            | Self::NonNull(keyword)
            | Self::Mixed(keyword)
            | Self::Resource(keyword)
            | Self::Never(keyword)
            | Self::Boolean(keyword)
            | Self::String(keyword) => vec![keyword],
            Self::Literal(literal) => vec![literal],
            Self::SignedInteger(signed) => vec![signed],
            Self::UnsignedInteger(unsigned) => vec![unsigned],
            Self::FloatingPoint(floating) => vec![floating],
            Self::Class(keyword, template)
            | Self::Interface(keyword, template)
            | Self::Iterable(keyword, template)
            | Self::Dict(keyword, template)
            | Self::Vec(keyword, template) => vec![keyword, template],
            Self::Tuple {
                type_definitions, ..
            } => type_definitions
                .inner
                .iter()
                .map(|t| t as &dyn Node)
                .collect::<Vec<&dyn Node>>(),
            Self::Parenthesized {
                type_definition, ..
            } => vec![type_definition.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Identifier(_) => "identifier type definition".to_string(),
            Self::Union(_) => "union type definition".to_string(),
            Self::Intersection(_) => "intersection type definition".to_string(),
            Self::Literal(literal) => literal.get_description(),
            Self::Nullable(_, _) => "nullable type definition".to_string(),
            Self::Void(_) => "void type definition".to_string(),
            Self::Never(_) => "never type definition".to_string(),
            Self::Boolean(_) => "boolean type definition".to_string(),
            Self::String(_) => "string type definition".to_string(),
            Self::Dict(_, _) => "dict type definition".to_string(),
            Self::Vec(_, _) => "vec type definition".to_string(),
            Self::Object(_) => "object type definition".to_string(),
            Self::Mixed(_) => "mixed type definition".to_string(),
            Self::NonNull(_) => "non-null type definition".to_string(),
            Self::Resource(_) => "resource type definition".to_string(),
            Self::Class(_, _) => "class type definition".to_string(),
            Self::Interface(_, _) => "interface type definition".to_string(),
            Self::Iterable(_, _) => "iterable type definition".to_string(),
            Self::SignedInteger(signed) => signed.get_description(),
            Self::UnsignedInteger(unsigned) => unsigned.get_description(),
            Self::FloatingPoint(floating) => floating.get_description(),
            Self::Tuple { .. } => "tuple type definition".to_string(),
            Self::Parenthesized { .. } => "parenthesized type definition".to_string(),
        }
    }
}

impl std::fmt::Display for TypeAliasDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.r#type, self.name, self.type_definition,
        )
    }
}

impl std::fmt::Display for TypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Identifier(inner) => write!(f, "{inner}"),
            Self::Nullable(_, inner) => write!(f, "?{inner}"),
            Self::Union(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join("|")
            ),
            Self::Intersection(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join("&")
            ),
            Self::Void(_) => write!(f, "void"),
            Self::Never(_) => write!(f, "never"),
            Self::Boolean(_) => write!(f, "bool"),
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::SignedInteger(signed) => write!(f, "{signed}"),
            Self::UnsignedInteger(unsigned) => write!(f, "{unsigned}"),
            Self::FloatingPoint(floating) => write!(f, "{floating}"),
            Self::String(_) => write!(f, "string"),
            Self::Dict(_, template) => write!(f, "dict{template}"),
            Self::Vec(_, template) => write!(f, "vec{template}"),
            Self::Iterable(_, template) => write!(f, "iterable{template}"),
            Self::Class(_, template) => write!(f, "class{template}"),
            Self::Interface(_, template) => write!(f, "interface{template}"),
            Self::Object(_) => write!(f, "object"),
            Self::Mixed(_) => write!(f, "mixed"),
            Self::NonNull(_) => write!(f, "nonnull"),
            Self::Resource(_) => write!(f, "resource"),
            Self::Tuple {
                type_definitions, ..
            } => write!(
                f,
                "({})",
                type_definitions
                    .inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Parenthesized {
                type_definition, ..
            } => {
                write!(f, "({type_definition})")
            }
        }
    }
}

impl std::fmt::Display for SignedIntegerTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(_) => write!(f, "int"),
            Self::I128(_) => write!(f, "i128"),
            Self::I64(_) => write!(f, "i64"),
            Self::I32(_) => write!(f, "i32"),
            Self::I16(_) => write!(f, "i16"),
            Self::I8(_) => write!(f, "i8"),
        }
    }
}

impl std::fmt::Display for UnsignedIntegerTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(_) => write!(f, "uint"),
            Self::U32(_) => write!(f, "u32"),
            Self::U16(_) => write!(f, "u16"),
            Self::U8(_) => write!(f, "u8"),
        }
    }
}

impl std::fmt::Display for FloatingPointTypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(_) => write!(f, "float"),
            Self::F64(_) => write!(f, "f64"),
            Self::F32(_) => write!(f, "f32"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::comment::CommentGroup;
    use crate::tree::identifier::Identifier;

    #[test]
    fn test_type_alias_definition_display() {
        let type_alias_definition = TypeAliasDefinition {
            r#type: Keyword {
                value: ByteString::from("type"),
                position: 0,
            },
            name: TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("Foo"),
                },
                templates: None,
            },
            equals: 0,
            type_definition: TypeDefinition::UnsignedInteger(UnsignedIntegerTypeDefinition::U32(
                Keyword {
                    value: ByteString::from("u32"),
                    position: 0,
                },
            )),
            semicolon: 0,
        };

        assert_eq!(type_alias_definition.to_string(), "type Foo = u32;");
    }

    #[test]
    fn test_type_alias_definition_with_templates_display() {
        let type_alias_definition = TypeAliasDefinition {
            r#type: Keyword {
                value: ByteString::from("type"),
                position: 0,
            },
            name: TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("filter"),
                },
                templates: Some(TypeTemplateGroupDefinition {
                    comments: CommentGroup { comments: vec![] },
                    less_than: 0,
                    members: CommaSeparated {
                        inner: vec![TypeDefinition::Identifier(TemplatedIdentifier {
                            name: Identifier {
                                position: 3,
                                value: ByteString::from("T"),
                            },
                            templates: None,
                        })],
                        commas: vec![],
                    },
                    greater_than: 0,
                }),
            },
            equals: 0,
            type_definition: TypeDefinition::Identifier(TemplatedIdentifier {
                name: Identifier {
                    position: 3,
                    value: ByteString::from("Closure"),
                },
                templates: Some(TypeTemplateGroupDefinition {
                    comments: CommentGroup { comments: vec![] },
                    less_than: 0,
                    members: CommaSeparated {
                        inner: vec![
                            TypeDefinition::Tuple {
                                left_parenthesis: 0,
                                type_definitions: CommaSeparated {
                                    inner: vec![TypeDefinition::Identifier(TemplatedIdentifier {
                                        name: Identifier {
                                            position: 3,
                                            value: ByteString::from("T"),
                                        },
                                        templates: None,
                                    })],
                                    commas: vec![],
                                },
                                right_parenthesis: 0,
                            },
                            TypeDefinition::Boolean(Keyword {
                                value: ByteString::from("bool"),
                                position: 0,
                            }),
                        ],
                        commas: vec![0],
                    },
                    greater_than: 0,
                }),
            }),
            semicolon: 0,
        };

        assert_eq!(
            type_alias_definition.to_string(),
            "type filter<T> = Closure<(T), bool>;"
        );
    }

    #[test]
    fn test_type_definition_display() {
        let type_definition = TypeDefinition::Identifier(TemplatedIdentifier {
            name: Identifier {
                position: 0,
                value: ByteString::from("Foo"),
            },
            templates: None,
        });

        assert_eq!(type_definition.to_string(), "Foo");

        let nullable = TypeDefinition::Nullable(
            0,
            Box::new(TypeDefinition::Identifier(TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("Foo"),
                },
                templates: None,
            })),
        );

        assert_eq!(nullable.to_string(), "?Foo");

        let union = TypeDefinition::Union(vec![
            TypeDefinition::Identifier(TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("Foo"),
                },
                templates: None,
            }),
            TypeDefinition::Identifier(TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("Bar"),
                },
                templates: None,
            }),
        ]);

        assert_eq!(union.to_string(), "Foo|Bar");

        let intersection = TypeDefinition::Intersection(vec![
            TypeDefinition::Identifier(TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("Foo"),
                },
                templates: None,
            }),
            TypeDefinition::Identifier(TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("Bar"),
                },
                templates: None,
            }),
        ]);

        assert_eq!(intersection.to_string(), "Foo&Bar");

        let void = TypeDefinition::Void(Keyword {
            value: ByteString::from("void"),
            position: 0,
        });

        assert_eq!(void.to_string(), "void");

        let never = TypeDefinition::Never(Keyword {
            value: ByteString::from("never"),
            position: 0,
        });

        assert_eq!(never.to_string(), "never");

        let boolean = TypeDefinition::Boolean(Keyword {
            value: ByteString::from("bool"),
            position: 0,
        });

        assert_eq!(boolean.to_string(), "bool");

        let string = TypeDefinition::String(Keyword {
            value: ByteString::from("string"),
            position: 0,
        });

        assert_eq!(string.to_string(), "string");

        let dict = TypeDefinition::Dict(
            Keyword {
                value: ByteString::from("dict"),
                position: 0,
            },
            TypeTemplateGroupDefinition {
                comments: CommentGroup { comments: vec![] },
                less_than: 0,
                members: CommaSeparated {
                    inner: vec![
                        TypeDefinition::Identifier(TemplatedIdentifier {
                            name: Identifier {
                                position: 0,
                                value: ByteString::from("Foo"),
                            },
                            templates: None,
                        }),
                        TypeDefinition::Identifier(TemplatedIdentifier {
                            name: Identifier {
                                position: 0,
                                value: ByteString::from("Bar"),
                            },
                            templates: None,
                        }),
                    ],
                    commas: vec![0],
                },
                greater_than: 0,
            },
        );

        assert_eq!(dict.to_string(), "dict<Foo, Bar>");

        let vec = TypeDefinition::Vec(
            Keyword {
                value: ByteString::from("vec"),
                position: 0,
            },
            TypeTemplateGroupDefinition {
                comments: CommentGroup { comments: vec![] },
                less_than: 0,
                members: CommaSeparated {
                    inner: vec![TypeDefinition::Identifier(TemplatedIdentifier {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Foo"),
                        },
                        templates: None,
                    })],
                    commas: vec![],
                },
                greater_than: 0,
            },
        );

        assert_eq!(vec.to_string(), "vec<Foo>");

        let object = TypeDefinition::Object(Keyword {
            value: ByteString::from("object"),
            position: 0,
        });

        assert_eq!(object.to_string(), "object");

        let mixed = TypeDefinition::Mixed(Keyword {
            value: ByteString::from("mixed"),
            position: 0,
        });

        assert_eq!(mixed.to_string(), "mixed");

        let nonnull = TypeDefinition::NonNull(Keyword {
            value: ByteString::from("nonnull"),
            position: 0,
        });

        assert_eq!(nonnull.to_string(), "nonnull");

        let resource = TypeDefinition::Resource(Keyword {
            value: ByteString::from("resource"),
            position: 0,
        });

        assert_eq!(resource.to_string(), "resource");

        let iterable = TypeDefinition::Iterable(
            Keyword {
                value: ByteString::from("iterable"),
                position: 0,
            },
            TypeTemplateGroupDefinition {
                comments: CommentGroup { comments: vec![] },
                less_than: 0,
                members: CommaSeparated {
                    inner: vec![TypeDefinition::Identifier(TemplatedIdentifier {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Foo"),
                        },
                        templates: None,
                    })],
                    commas: vec![],
                },
                greater_than: 0,
            },
        );

        assert_eq!(iterable.to_string(), "iterable<Foo>");

        let class = TypeDefinition::Class(
            Keyword {
                value: ByteString::from("class"),
                position: 0,
            },
            TypeTemplateGroupDefinition {
                comments: CommentGroup { comments: vec![] },
                less_than: 0,
                members: CommaSeparated {
                    inner: vec![TypeDefinition::Identifier(TemplatedIdentifier {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Foo"),
                        },
                        templates: None,
                    })],
                    commas: vec![],
                },
                greater_than: 0,
            },
        );

        assert_eq!(class.to_string(), "class<Foo>");

        let interface = TypeDefinition::Interface(
            Keyword {
                value: ByteString::from("interface"),
                position: 0,
            },
            TypeTemplateGroupDefinition {
                comments: CommentGroup { comments: vec![] },
                less_than: 0,
                members: CommaSeparated {
                    inner: vec![TypeDefinition::Identifier(TemplatedIdentifier {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Foo"),
                        },
                        templates: None,
                    })],
                    commas: vec![],
                },
                greater_than: 0,
            },
        );

        assert_eq!(interface.to_string(), "interface<Foo>");

        let tuple = TypeDefinition::Tuple {
            left_parenthesis: 0,
            type_definitions: CommaSeparated {
                inner: vec![
                    TypeDefinition::Identifier(TemplatedIdentifier {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Foo"),
                        },
                        templates: None,
                    }),
                    TypeDefinition::Identifier(TemplatedIdentifier {
                        name: Identifier {
                            position: 0,
                            value: ByteString::from("Bar"),
                        },
                        templates: None,
                    }),
                ],
                commas: vec![0],
            },
            right_parenthesis: 0,
        };

        assert_eq!(tuple.to_string(), "(Foo, Bar)");

        let parenthesized = TypeDefinition::Parenthesized {
            left_parenthesis: 0,
            type_definition: Box::new(TypeDefinition::Identifier(TemplatedIdentifier {
                name: Identifier {
                    position: 0,
                    value: ByteString::from("Foo"),
                },
                templates: None,
            })),
            right_parenthesis: 0,
        };

        assert_eq!(parenthesized.to_string(), "(Foo)");
    }
}
