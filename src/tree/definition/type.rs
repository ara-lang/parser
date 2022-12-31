use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::definition::template::TypeTemplateGroupDefinition;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypeAliasDefinition {
    pub r#type: usize,
    pub name: TemplatedIdentifier,
    pub equals: usize,
    pub data_type: TypeDefinition,
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
    Null(usize),
    True(usize),
    False(usize),
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
    Function {
        outer_left_parenthesis: usize,
        r#fn: usize,
        left_parenthesis: usize,
        parameter_type_definitions: CommaSeparated<TypeDefinition>,
        right_parenthesis: usize,
        colon: usize,
        return_type_definition: Box<TypeDefinition>,
        outer_right_parenthesis: usize,
    },
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
        )
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
        vec![&self.name, &self.data_type]
    }
}

impl Node for TypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            TypeDefinition::Identifier(inner) => inner.initial_position(),
            TypeDefinition::Nullable(position, _) => *position,
            TypeDefinition::Union(inner) => inner[0].initial_position(),
            TypeDefinition::Intersection(inner) => inner[0].initial_position(),
            TypeDefinition::Void(position) => *position,
            TypeDefinition::Null(position) => *position,
            TypeDefinition::True(position) => *position,
            TypeDefinition::False(position) => *position,
            TypeDefinition::Never(position) => *position,
            TypeDefinition::Float(position) => *position,
            TypeDefinition::Boolean(position) => *position,
            TypeDefinition::Integer(position) => *position,
            TypeDefinition::String(position) => *position,
            TypeDefinition::Dict(position, _) => *position,
            TypeDefinition::Vec(position, _) => *position,
            TypeDefinition::Object(position) => *position,
            TypeDefinition::Mixed(position) => *position,
            TypeDefinition::NonNull(position) => *position,
            TypeDefinition::Resource(position) => *position,
            TypeDefinition::Iterable(position, _) => *position,
            TypeDefinition::Function {
                outer_left_parenthesis,
                ..
            } => *outer_left_parenthesis,
            TypeDefinition::Tuple {
                left_parenthesis, ..
            } => *left_parenthesis,
            TypeDefinition::Parenthesized {
                left_parenthesis, ..
            } => *left_parenthesis,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            TypeDefinition::Identifier(inner) => inner.final_position(),
            TypeDefinition::Nullable(_, inner) => inner.final_position(),
            TypeDefinition::Union(inner) => inner[inner.len() - 1].final_position(),
            TypeDefinition::Intersection(inner) => inner[inner.len() - 1].final_position(),
            TypeDefinition::Void(position) => position + 4,
            TypeDefinition::Null(position) => position + 4,
            TypeDefinition::True(position) => position + 4,
            TypeDefinition::False(position) => position + 5,
            TypeDefinition::Never(position) => position + 5,
            TypeDefinition::Float(position) => position + 5,
            TypeDefinition::Boolean(position) => position + 7,
            TypeDefinition::Integer(position) => position + 7,
            TypeDefinition::String(position) => position + 6,
            TypeDefinition::Dict(_, template) => template.final_position(),
            TypeDefinition::Vec(_, template) => template.final_position(),
            TypeDefinition::Object(position) => position + 6,
            TypeDefinition::Mixed(position) => position + 5,
            TypeDefinition::NonNull(position) => position + 7,
            TypeDefinition::Resource(position) => position + 8,
            TypeDefinition::Iterable(_, template) => template.final_position(),
            TypeDefinition::Function {
                outer_right_parenthesis,
                ..
            } => outer_right_parenthesis + 1,
            TypeDefinition::Tuple {
                right_parenthesis, ..
            } => right_parenthesis + 1,
            TypeDefinition::Parenthesized {
                right_parenthesis, ..
            } => right_parenthesis + 1,
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
