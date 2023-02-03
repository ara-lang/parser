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
            // match &self and print all the variants:
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
            Self::Literal(literal) => match literal {
                Literal::Null(_) => write!(f, "null"),
                Literal::False(_) => write!(f, "false"),
                Literal::True(_) => write!(f, "true"),
                Literal::Integer(inner) => write!(f, "{}", inner.value),
                Literal::Float(inner) => write!(f, "{}", inner.value),
                Literal::String(inner) => write!(f, "{}", inner.value),
            },
            Self::SignedInteger(signed) => match signed {
                SignedIntegerTypeDefinition::Default(_) => write!(f, "int"),
                SignedIntegerTypeDefinition::I128(_) => write!(f, "i128"),
                SignedIntegerTypeDefinition::I64(_) => write!(f, "i64"),
                SignedIntegerTypeDefinition::I32(_) => write!(f, "i32"),
                SignedIntegerTypeDefinition::I16(_) => write!(f, "i16"),
                SignedIntegerTypeDefinition::I8(_) => write!(f, "i8"),
            },
            Self::UnsignedInteger(unsigned) => match unsigned {
                UnsignedIntegerTypeDefinition::Default(_) => write!(f, "uint"),
                UnsignedIntegerTypeDefinition::U32(_) => write!(f, "u32"),
                UnsignedIntegerTypeDefinition::U16(_) => write!(f, "u16"),
                UnsignedIntegerTypeDefinition::U8(_) => write!(f, "u8"),
            },
            Self::FloatingPoint(floating) => match floating {
                FloatingPointTypeDefinition::Default(_) => write!(f, "float"),
                FloatingPointTypeDefinition::F64(_) => write!(f, "f64"),
                FloatingPointTypeDefinition::F32(_) => write!(f, "f32"),
            },
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
