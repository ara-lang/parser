use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::expression::literal::Literal;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeAliasDefinition {
    pub r#type: Keyword,
    pub name: TemplatedIdentifier,
    pub equals: usize,
    pub type_definition: TypeDefinition,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TypeDefinition {
    Identifier(TemplatedIdentifier),
    Nullable(usize, Box<TypeDefinition>),
    Union(Vec<TypeDefinition>),
    Intersection(Vec<TypeDefinition>),
    Void(Keyword),
    Never(Keyword),
    Float(Keyword),
    Boolean(Keyword),
    Integer(Keyword),
    String(Keyword),
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
            | TypeDefinition::Float(_)
            | TypeDefinition::Boolean(_)
            | TypeDefinition::Integer(_)
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
            | TypeDefinition::Float(keyword)
            | TypeDefinition::Boolean(keyword)
            | TypeDefinition::Integer(keyword)
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
            | TypeDefinition::Float(keyword)
            | TypeDefinition::Boolean(keyword)
            | TypeDefinition::Integer(keyword)
            | TypeDefinition::String(keyword)
            | TypeDefinition::Object(keyword)
            | TypeDefinition::Mixed(keyword)
            | TypeDefinition::NonNull(keyword)
            | TypeDefinition::Resource(keyword) => keyword.final_position(),
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
            | TypeDefinition::Float(keyword)
            | TypeDefinition::Boolean(keyword)
            | TypeDefinition::Integer(keyword)
            | TypeDefinition::String(keyword) => vec![keyword],
            TypeDefinition::Literal(literal) => vec![literal],
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
            TypeDefinition::Float(_) => write!(f, "float"),
            TypeDefinition::Boolean(_) => write!(f, "bool"),
            TypeDefinition::Integer(_) => write!(f, "int"),
            TypeDefinition::Literal(literal) => match literal {
                Literal::Null(_) => write!(f, "null"),
                Literal::False(_) => write!(f, "false"),
                Literal::True(_) => write!(f, "true"),
                Literal::Integer(inner) => write!(f, "{}", inner.value),
                Literal::Float(inner) => write!(f, "{}", inner.value),
                Literal::String(inner) => write!(f, "{}", inner.value),
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
