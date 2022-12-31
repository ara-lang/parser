use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeAliasDefinition {
    pub r#type: Span,
    pub name: TemplatedIdentifier,
    pub equals: Span,
    pub data_type: TypeDefinition,
    pub semicolon: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TypeDefinition {
    Identifier(TemplatedIdentifier),
    Nullable(Span, Box<TypeDefinition>),
    Union(Vec<TypeDefinition>),
    Intersection(Vec<TypeDefinition>),
    Void(Span),
    Null(Span),
    True(Span),
    False(Span),
    Never(Span),
    Float(Span),
    Boolean(Span),
    Integer(Span),
    String(Span),
    Dict(Span, TypeTemplateGroupDefinition),
    Vec(Span, TypeTemplateGroupDefinition),
    Object(Span),
    Mixed(Span),
    NonNull(Span),
    Resource(Span),
    Iterable(Span, TypeTemplateGroupDefinition),
    Function {
        outer_left_parenthesis: Span,
        r#fn: Span,
        left_parenthesis: Span,
        parameter_type_definitions: CommaSeparated<TypeDefinition>,
        right_parenthesis: Span,
        colon: Span,
        return_type_definition: Box<TypeDefinition>,
        outer_right_parenthesis: Span,
    },
    Tuple {
        left_parenthesis: Span,
        type_definitions: CommaSeparated<TypeDefinition>,
        right_parenthesis: Span,
    },
    Parenthesized {
        left_parenthesis: Span,
        type_definition: Box<TypeDefinition>,
        right_parenthesis: Span,
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
        )
    }
}

impl Node for TypeAliasDefinition {
    fn initial_position(&self) -> usize {
        self.r#type.position
    }

    fn final_position(&self) -> usize {
        self.semicolon.position + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.name, &self.data_type]
    }
}

impl Node for TypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            TypeDefinition::Identifier(inner) => inner.initial_position(),
            TypeDefinition::Nullable(span, _) => span.position,
            TypeDefinition::Union(inner) => inner[0].initial_position(),
            TypeDefinition::Intersection(inner) => inner[0].initial_position(),
            TypeDefinition::Void(span) => span.position,
            TypeDefinition::Null(span) => span.position,
            TypeDefinition::True(span) => span.position,
            TypeDefinition::False(span) => span.position,
            TypeDefinition::Never(span) => span.position,
            TypeDefinition::Float(span) => span.position,
            TypeDefinition::Boolean(span) => span.position,
            TypeDefinition::Integer(span) => span.position,
            TypeDefinition::String(span) => span.position,
            TypeDefinition::Dict(span, _) => span.position,
            TypeDefinition::Vec(span, _) => span.position,
            TypeDefinition::Object(span) => span.position,
            TypeDefinition::Mixed(span) => span.position,
            TypeDefinition::NonNull(span) => span.position,
            TypeDefinition::Resource(span) => span.position,
            TypeDefinition::Iterable(span, _) => span.position,
            TypeDefinition::Function {
                outer_left_parenthesis,
                ..
            } => outer_left_parenthesis.position,
            TypeDefinition::Tuple {
                left_parenthesis, ..
            } => left_parenthesis.position,
            TypeDefinition::Parenthesized {
                left_parenthesis, ..
            } => left_parenthesis.position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            TypeDefinition::Identifier(inner) => inner.final_position(),
            TypeDefinition::Nullable(_, inner) => inner.final_position(),
            TypeDefinition::Union(inner) => inner[inner.len() - 1].final_position(),
            TypeDefinition::Intersection(inner) => inner[inner.len() - 1].final_position(),
            TypeDefinition::Void(span) => span.position + 4,
            TypeDefinition::Null(span) => span.position + 4,
            TypeDefinition::True(span) => span.position + 4,
            TypeDefinition::False(span) => span.position + 5,
            TypeDefinition::Never(span) => span.position + 5,
            TypeDefinition::Float(span) => span.position + 5,
            TypeDefinition::Boolean(span) => span.position + 7,
            TypeDefinition::Integer(span) => span.position + 7,
            TypeDefinition::String(span) => span.position + 6,
            TypeDefinition::Dict(_, template) => template.final_position(),
            TypeDefinition::Vec(_, template) => template.final_position(),
            TypeDefinition::Object(span) => span.position + 6,
            TypeDefinition::Mixed(span) => span.position + 5,
            TypeDefinition::NonNull(span) => span.position + 7,
            TypeDefinition::Resource(span) => span.position + 8,
            TypeDefinition::Iterable(_, template) => template.final_position(),
            TypeDefinition::Function {
                outer_right_parenthesis,
                ..
            } => outer_right_parenthesis.position + 1,
            TypeDefinition::Tuple {
                right_parenthesis, ..
            } => right_parenthesis.position + 1,
            TypeDefinition::Parenthesized {
                right_parenthesis, ..
            } => right_parenthesis.position + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            TypeDefinition::Identifier(inner) => vec![inner],
            TypeDefinition::Nullable(_, inner) => vec![inner.as_ref()],
            TypeDefinition::Union(inner) => inner.iter().map(|t| t as &dyn Node).collect(),
            TypeDefinition::Intersection(inner) => inner.iter().map(|t| t as &dyn Node).collect(),
            TypeDefinition::Void(_) => vec![],
            TypeDefinition::Null(_) => vec![],
            TypeDefinition::True(_) => vec![],
            TypeDefinition::False(_) => vec![],
            TypeDefinition::Never(_) => vec![],
            TypeDefinition::Float(_) => vec![],
            TypeDefinition::Boolean(_) => vec![],
            TypeDefinition::Integer(_) => vec![],
            TypeDefinition::String(_) => vec![],
            TypeDefinition::Dict(_, template) => vec![template],
            TypeDefinition::Vec(_, template) => vec![template],
            TypeDefinition::Object(_) => vec![],
            TypeDefinition::Mixed(_) => vec![],
            TypeDefinition::NonNull(_) => vec![],
            TypeDefinition::Resource(_) => vec![],
            TypeDefinition::Iterable(_, template) => vec![template],
            TypeDefinition::Function {
                parameter_type_definitions,
                return_type_definition,
                ..
            } => {
                let mut children = parameter_type_definitions
                    .inner
                    .iter()
                    .map(|t| t as &dyn Node)
                    .collect::<Vec<&dyn Node>>();

                children.push(return_type_definition.as_ref());
                children
            }
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
            TypeDefinition::Null(_) => write!(f, "null"),
            TypeDefinition::True(_) => write!(f, "true"),
            TypeDefinition::False(_) => write!(f, "false"),
            TypeDefinition::Never(_) => write!(f, "never"),
            TypeDefinition::Float(_) => write!(f, "float"),
            TypeDefinition::Boolean(_) => write!(f, "bool"),
            TypeDefinition::Integer(_) => write!(f, "int"),
            TypeDefinition::String(_) => write!(f, "string"),
            TypeDefinition::Dict(_, template) => write!(f, "dict{}", template),
            TypeDefinition::Vec(_, template) => write!(f, "vec{}", template),
            TypeDefinition::Iterable(_, template) => write!(f, "iterable{}", template),
            TypeDefinition::Object(_) => write!(f, "object"),
            TypeDefinition::Mixed(_) => write!(f, "mixed"),
            TypeDefinition::NonNull(_) => write!(f, "nonnull"),
            TypeDefinition::Resource(_) => write!(f, "resource"),
            TypeDefinition::Function {
                parameter_type_definitions: parameters,
                return_type_definition: return_type,
                ..
            } => write!(
                f,
                "(fn({}): {})",
                parameters
                    .inner
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                return_type
            ),
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
