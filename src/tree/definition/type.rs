use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::expression::literal::Literal;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeAliasDefinition {
    pub r#type: usize,
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
    Void(usize),
    Never(usize),
    Float(usize),
    Boolean(usize),
    Integer(usize),
    String(usize),
    Dict(usize, TypeTemplateGroupDefinition),
    Vec(usize, TypeTemplateGroupDefinition),
    Object(usize),
    Mixed(usize),
    NonNull(usize),
    Resource(usize),
    Iterable(usize, TypeTemplateGroupDefinition),
    Class(usize, TypeTemplateGroupDefinition),
    Interface(usize, TypeTemplateGroupDefinition),
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
        matches!(
            self,
            TypeDefinition::Float(_)
                | TypeDefinition::Boolean(_)
                | TypeDefinition::Integer(_)
                | TypeDefinition::String(_)
                // class, and interface are represented as strings at runtime, so they are considered scalars
                | TypeDefinition::Class(_, _)
                | TypeDefinition::Interface(_, _)
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, TypeDefinition::Literal(_))
    }
}

impl Node for TypeAliasDefinition {
    fn initial_position(&self) -> usize {
        self.r#type
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.name, &self.type_definition]
    }
}

impl Node for TypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            TypeDefinition::Identifier(inner) => inner.initial_position(),
            TypeDefinition::Union(inner) => inner[0].initial_position(),
            TypeDefinition::Intersection(inner) => inner[0].initial_position(),
            TypeDefinition::Literal(literal) => literal.initial_position(),
            TypeDefinition::Nullable(position, _)
            | TypeDefinition::Void(position)
            | TypeDefinition::Never(position)
            | TypeDefinition::Float(position)
            | TypeDefinition::Boolean(position)
            | TypeDefinition::Integer(position)
            | TypeDefinition::String(position)
            | TypeDefinition::Dict(position, _)
            | TypeDefinition::Vec(position, _)
            | TypeDefinition::Object(position)
            | TypeDefinition::Mixed(position)
            | TypeDefinition::NonNull(position)
            | TypeDefinition::Resource(position)
            | TypeDefinition::Class(position, _)
            | TypeDefinition::Interface(position, _)
            | TypeDefinition::Iterable(position, _)
            | TypeDefinition::Tuple {
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
            TypeDefinition::Void(position) => position + 4,
            TypeDefinition::Never(position) => position + 5,
            TypeDefinition::Float(position) => position + 5,
            TypeDefinition::Boolean(position) => position + 7,
            TypeDefinition::Integer(position) => position + 7,
            TypeDefinition::String(position) => position + 6,
            TypeDefinition::Object(position) => position + 6,
            TypeDefinition::Mixed(position) => position + 5,
            TypeDefinition::NonNull(position) => position + 7,
            TypeDefinition::Resource(position) => position + 8,
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
            TypeDefinition::Void(_)
            | TypeDefinition::Object(_)
            | TypeDefinition::NonNull(_)
            | TypeDefinition::Mixed(_)
            | TypeDefinition::Resource(_)
            | TypeDefinition::Never(_)
            | TypeDefinition::Float(_)
            | TypeDefinition::Boolean(_)
            | TypeDefinition::Integer(_)
            | TypeDefinition::String(_) => vec![],
            TypeDefinition::Literal(literal) => vec![literal],
            TypeDefinition::Class(_, template)
            | TypeDefinition::Interface(_, template)
            | TypeDefinition::Iterable(_, template)
            | TypeDefinition::Dict(_, template)
            | TypeDefinition::Vec(_, template) => vec![template],
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
