use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::expression::literal::Literal;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeAliasDefinition {
    pub r#type: Keyword,
    pub name: TemplatedIdentifier,
    pub equals: usize,
    pub type_definition: TypeDefinition,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum SignedIntegerTypeDefinition {
    Default(Keyword), // 'int'
    I128(Keyword),    // 'i128'
    I64(Keyword),     // 'i64'
    I32(Keyword),     // 'i32'
    I16(Keyword),     // 'i16'
    I8(Keyword),      // 'i8'
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum UnsignedIntegerTypeDefinition {
    Default(Keyword), // 'uint'
    U32(Keyword),     // 'u32'
    U16(Keyword),     // 'u16'
    U8(Keyword),      // 'u8'
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum FloatingPointTypeDefinition {
    Default(Keyword), // 'float'
    F64(Keyword),     // 'f64'
    F32(Keyword),     // 'f32'
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
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
            TypeDefinition::Mixed(_)
                | TypeDefinition::Never(_)
                | TypeDefinition::Void(_)
                | TypeDefinition::Nullable(_, _)
                | TypeDefinition::NonNull(_)
                | TypeDefinition::Resource(_)
        )
    }

    pub fn is_nullable(&self) -> bool {
        matches!(self, TypeDefinition::Nullable(_, _))
    }

    pub fn is_bottom(&self) -> bool {
        matches!(self, TypeDefinition::Never(_) | TypeDefinition::Void(_))
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            TypeDefinition::Literal(literal) => !matches!(literal, Literal::Null(_)),
            | TypeDefinition::Boolean(_)
            | TypeDefinition::SignedInteger(_)
            | TypeDefinition::UnsignedInteger(_)
            | TypeDefinition::FloatingPoint(_)
            | TypeDefinition::String(_)
            // class, and interface are represented as strings at runtime, so they are considered scalars
            | TypeDefinition::Class(_, _)
            | TypeDefinition::Interface(_, _) => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, TypeDefinition::Literal(_))
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
        match self {
            SignedIntegerTypeDefinition::Default(keyword)
            | SignedIntegerTypeDefinition::I128(keyword)
            | SignedIntegerTypeDefinition::I64(keyword)
            | SignedIntegerTypeDefinition::I32(keyword)
            | SignedIntegerTypeDefinition::I16(keyword)
            | SignedIntegerTypeDefinition::I8(keyword) => keyword.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            SignedIntegerTypeDefinition::Default(keyword)
            | SignedIntegerTypeDefinition::I128(keyword)
            | SignedIntegerTypeDefinition::I64(keyword)
            | SignedIntegerTypeDefinition::I32(keyword)
            | SignedIntegerTypeDefinition::I16(keyword)
            | SignedIntegerTypeDefinition::I8(keyword) => keyword.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            SignedIntegerTypeDefinition::Default(keyword)
            | SignedIntegerTypeDefinition::I128(keyword)
            | SignedIntegerTypeDefinition::I64(keyword)
            | SignedIntegerTypeDefinition::I32(keyword)
            | SignedIntegerTypeDefinition::I16(keyword)
            | SignedIntegerTypeDefinition::I8(keyword) => vec![keyword],
        }
    }

    fn get_description(&self) -> String {
        "signed integer type definition".to_string()
    }
}

impl Node for UnsignedIntegerTypeDefinition {
    fn initial_position(&self) -> usize {
        match self {
            UnsignedIntegerTypeDefinition::Default(keyword)
            | UnsignedIntegerTypeDefinition::U32(keyword)
            | UnsignedIntegerTypeDefinition::U16(keyword)
            | UnsignedIntegerTypeDefinition::U8(keyword) => keyword.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            UnsignedIntegerTypeDefinition::Default(keyword)
            | UnsignedIntegerTypeDefinition::U32(keyword)
            | UnsignedIntegerTypeDefinition::U16(keyword)
            | UnsignedIntegerTypeDefinition::U8(keyword) => keyword.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            UnsignedIntegerTypeDefinition::Default(keyword)
            | UnsignedIntegerTypeDefinition::U32(keyword)
            | UnsignedIntegerTypeDefinition::U16(keyword)
            | UnsignedIntegerTypeDefinition::U8(keyword) => vec![keyword],
        }
    }

    fn get_description(&self) -> String {
        "unsigned integer type definition".to_string()
    }
}

impl Node for FloatingPointTypeDefinition {
    fn initial_position(&self) -> usize {
        match self {
            FloatingPointTypeDefinition::Default(keyword)
            | FloatingPointTypeDefinition::F64(keyword)
            | FloatingPointTypeDefinition::F32(keyword) => keyword.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            FloatingPointTypeDefinition::Default(keyword)
            | FloatingPointTypeDefinition::F64(keyword)
            | FloatingPointTypeDefinition::F32(keyword) => keyword.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            FloatingPointTypeDefinition::Default(keyword)
            | FloatingPointTypeDefinition::F64(keyword)
            | FloatingPointTypeDefinition::F32(keyword) => vec![keyword],
        }
    }

    fn get_description(&self) -> String {
        "floating point type definition".to_string()
    }
}

impl Node for TypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            TypeDefinition::Identifier(inner) => inner.initial_position(),
            TypeDefinition::Union(inner) => inner[0].initial_position(),
            TypeDefinition::Intersection(inner) => inner[0].initial_position(),
            TypeDefinition::Literal(literal) => literal.initial_position(),
            TypeDefinition::Nullable(position, _) => *position,
            TypeDefinition::Void(keyword)
            | TypeDefinition::Never(keyword)
            | TypeDefinition::Boolean(keyword)
            | TypeDefinition::String(keyword)
            | TypeDefinition::Dict(keyword, _)
            | TypeDefinition::Vec(keyword, _)
            | TypeDefinition::Object(keyword)
            | TypeDefinition::Mixed(keyword)
            | TypeDefinition::NonNull(keyword)
            | TypeDefinition::Resource(keyword)
            | TypeDefinition::Class(keyword, _)
            | TypeDefinition::Interface(keyword, _)
            | TypeDefinition::Iterable(keyword, _) => keyword.initial_position(),
            TypeDefinition::SignedInteger(signed) => signed.initial_position(),
            TypeDefinition::UnsignedInteger(unsigned) => unsigned.initial_position(),
            TypeDefinition::FloatingPoint(floating) => floating.initial_position(),
            TypeDefinition::Tuple {
                left_parenthesis: position,
                ..
            }
            | TypeDefinition::Parenthesized {
                left_parenthesis: position,
                ..
            } => *position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            TypeDefinition::Identifier(inner) => inner.final_position(),
            TypeDefinition::Nullable(_, inner) => inner.final_position(),
            TypeDefinition::Dict(_, template)
            | TypeDefinition::Vec(_, template)
            | TypeDefinition::Class(_, template)
            | TypeDefinition::Interface(_, template)
            | TypeDefinition::Iterable(_, template) => template.final_position(),
            TypeDefinition::Union(inner) => inner[inner.len() - 1].final_position(),
            TypeDefinition::Intersection(inner) => inner[inner.len() - 1].final_position(),
            TypeDefinition::Literal(literal) => literal.final_position(),
            TypeDefinition::Void(keyword)
            | TypeDefinition::Never(keyword)
            | TypeDefinition::Boolean(keyword)
            | TypeDefinition::String(keyword)
            | TypeDefinition::Object(keyword)
            | TypeDefinition::Mixed(keyword)
            | TypeDefinition::NonNull(keyword)
            | TypeDefinition::Resource(keyword) => keyword.final_position(),
            TypeDefinition::SignedInteger(signed) => signed.final_position(),
            TypeDefinition::UnsignedInteger(unsigned) => unsigned.final_position(),
            TypeDefinition::FloatingPoint(floating) => floating.final_position(),
            TypeDefinition::Parenthesized {
                right_parenthesis, ..
            }
            | TypeDefinition::Tuple {
                right_parenthesis, ..
            } => right_parenthesis + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            TypeDefinition::Identifier(inner) => vec![inner],
            TypeDefinition::Nullable(_, inner) => vec![inner.as_ref()],
            TypeDefinition::Union(inner) | TypeDefinition::Intersection(inner) => {
                inner.iter().map(|t| t as &dyn Node).collect()
            }
            TypeDefinition::Void(keyword)
            | TypeDefinition::Object(keyword)
            | TypeDefinition::NonNull(keyword)
            | TypeDefinition::Mixed(keyword)
            | TypeDefinition::Resource(keyword)
            | TypeDefinition::Never(keyword)
            | TypeDefinition::Boolean(keyword)
            | TypeDefinition::String(keyword) => vec![keyword],
            TypeDefinition::Literal(literal) => vec![literal],
            TypeDefinition::SignedInteger(signed) => vec![signed],
            TypeDefinition::UnsignedInteger(unsigned) => vec![unsigned],
            TypeDefinition::FloatingPoint(floating) => vec![floating],
            TypeDefinition::Class(keyword, template)
            | TypeDefinition::Interface(keyword, template)
            | TypeDefinition::Iterable(keyword, template)
            | TypeDefinition::Dict(keyword, template)
            | TypeDefinition::Vec(keyword, template) => vec![keyword, template],
            TypeDefinition::Tuple {
                type_definitions, ..
            } => type_definitions
                .inner
                .iter()
                .map(|t| t as &dyn Node)
                .collect::<Vec<&dyn Node>>(),
            TypeDefinition::Parenthesized {
                type_definition, ..
            } => vec![type_definition.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            // match &self and print all the variants:
            TypeDefinition::Identifier(_) => "identifier type definition".to_string(),
            TypeDefinition::Union(_) => "union type definition".to_string(),
            TypeDefinition::Intersection(_) => "intersection type definition".to_string(),
            TypeDefinition::Literal(literal) => literal.get_description(),
            TypeDefinition::Nullable(_, _) => "nullable type definition".to_string(),
            TypeDefinition::Void(_) => "void type definition".to_string(),
            TypeDefinition::Never(_) => "never type definition".to_string(),
            TypeDefinition::Boolean(_) => "boolean type definition".to_string(),
            TypeDefinition::String(_) => "string type definition".to_string(),
            TypeDefinition::Dict(_, _) => "dict type definition".to_string(),
            TypeDefinition::Vec(_, _) => "vec type definition".to_string(),
            TypeDefinition::Object(_) => "object type definition".to_string(),
            TypeDefinition::Mixed(_) => "mixed type definition".to_string(),
            TypeDefinition::NonNull(_) => "non-null type definition".to_string(),
            TypeDefinition::Resource(_) => "resource type definition".to_string(),
            TypeDefinition::Class(_, _) => "class type definition".to_string(),
            TypeDefinition::Interface(_, _) => "interface type definition".to_string(),
            TypeDefinition::Iterable(_, _) => "iterable type definition".to_string(),
            TypeDefinition::SignedInteger(signed) => signed.get_description(),
            TypeDefinition::UnsignedInteger(unsigned) => unsigned.get_description(),
            TypeDefinition::FloatingPoint(floating) => floating.get_description(),
            TypeDefinition::Tuple { .. } => "tuple type definition".to_string(),
            TypeDefinition::Parenthesized { .. } => "parenthesized type definition".to_string(),
        }
    }
}

impl std::fmt::Display for TypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TypeDefinition::Identifier(inner) => write!(f, "{}", inner),
            TypeDefinition::Nullable(_, inner) => write!(f, "?{}", inner),
            TypeDefinition::Union(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join("|")
            ),
            TypeDefinition::Intersection(inner) => write!(
                f,
                "{}",
                inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join("&")
            ),
            TypeDefinition::Void(_) => write!(f, "void"),
            TypeDefinition::Never(_) => write!(f, "never"),
            TypeDefinition::Boolean(_) => write!(f, "bool"),
            TypeDefinition::Literal(literal) => match literal {
                Literal::Null(_) => write!(f, "null"),
                Literal::False(_) => write!(f, "false"),
                Literal::True(_) => write!(f, "true"),
                Literal::Integer(inner) => write!(f, "{}", inner.value),
                Literal::Float(inner) => write!(f, "{}", inner.value),
                Literal::String(inner) => write!(f, "{}", inner.value),
            },
            TypeDefinition::SignedInteger(signed) => match signed {
                SignedIntegerTypeDefinition::Default(_) => write!(f, "int"),
                SignedIntegerTypeDefinition::I128(_) => write!(f, "i128"),
                SignedIntegerTypeDefinition::I64(_) => write!(f, "i64"),
                SignedIntegerTypeDefinition::I32(_) => write!(f, "i32"),
                SignedIntegerTypeDefinition::I16(_) => write!(f, "i16"),
                SignedIntegerTypeDefinition::I8(_) => write!(f, "i8"),
            },
            TypeDefinition::UnsignedInteger(unsigned) => match unsigned {
                UnsignedIntegerTypeDefinition::Default(_) => write!(f, "uint"),
                UnsignedIntegerTypeDefinition::U32(_) => write!(f, "u32"),
                UnsignedIntegerTypeDefinition::U16(_) => write!(f, "u16"),
                UnsignedIntegerTypeDefinition::U8(_) => write!(f, "u8"),
            },
            TypeDefinition::FloatingPoint(floating) => match floating {
                FloatingPointTypeDefinition::Default(_) => write!(f, "float"),
                FloatingPointTypeDefinition::F64(_) => write!(f, "f64"),
                FloatingPointTypeDefinition::F32(_) => write!(f, "f32"),
            },
            TypeDefinition::String(_) => write!(f, "string"),
            TypeDefinition::Dict(_, template) => write!(f, "dict{}", template),
            TypeDefinition::Vec(_, template) => write!(f, "vec{}", template),
            TypeDefinition::Iterable(_, template) => write!(f, "iterable{}", template),
            TypeDefinition::Class(_, template) => write!(f, "class{}", template),
            TypeDefinition::Interface(_, template) => write!(f, "interface{}", template),
            TypeDefinition::Object(_) => write!(f, "object"),
            TypeDefinition::Mixed(_) => write!(f, "mixed"),
            TypeDefinition::NonNull(_) => write!(f, "nonnull"),
            TypeDefinition::Resource(_) => write!(f, "resource"),
            TypeDefinition::Tuple {
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
            TypeDefinition::Parenthesized {
                type_definition, ..
            } => {
                write!(f, "({})", type_definition)
            }
        }
    }
}
