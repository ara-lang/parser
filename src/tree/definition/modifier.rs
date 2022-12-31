use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::lexer::token::Span;
use crate::tree::Node;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum VisibilityModifierDefinition {
    Public(Span),
    Protected(Span),
    Private(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum PromotedPropertyModifierDefinition {
    Public(Span),
    Protected(Span),
    Private(Span),
    Readonly(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct PromotedPropertyModifierDefinitionGroup {
    pub modifiers: Vec<PromotedPropertyModifierDefinition>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum PropertyModifierDefinition {
    Public(Span),
    Protected(Span),
    Private(Span),
    Static(Span),
    Readonly(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct PropertyModifierDefinitionGroup {
    pub modifiers: Vec<PropertyModifierDefinition>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum MethodModifierDefinition {
    Final(Span),
    Static(Span),
    Abstract(Span),
    Public(Span),
    Protected(Span),
    Private(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct MethodModifierDefinitionGroup {
    pub modifiers: Vec<MethodModifierDefinition>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ClassModifierDefinition {
    Final(Span),
    Abstract(Span),
    Readonly(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct ClassModifierDefinitionGroup {
    pub modifiers: Vec<ClassModifierDefinition>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ConstantModifierDefinition {
    Final(Span),
    Public(Span),
    Protected(Span),
    Private(Span),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct ConstantModifierDefinitionGroup {
    pub modifiers: Vec<ConstantModifierDefinition>,
}

impl PromotedPropertyModifierDefinition {
    // TODO(azjezz): remove
    pub fn span(&self) -> Span {
        match self {
            PromotedPropertyModifierDefinition::Public(span) => *span,
            PromotedPropertyModifierDefinition::Protected(span) => *span,
            PromotedPropertyModifierDefinition::Private(span) => *span,
            PromotedPropertyModifierDefinition::Readonly(span) => *span,
        }
    }
}

impl PromotedPropertyModifierDefinitionGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn get_readonly(&self) -> Option<&PromotedPropertyModifierDefinition> {
        self.modifiers.iter().find(|modifier| {
            matches!(
                modifier,
                PromotedPropertyModifierDefinition::Readonly { .. }
            )
        })
    }

    pub fn has_readonly(&self) -> bool {
        self.modifiers.iter().any(|modifier| {
            matches!(
                modifier,
                PromotedPropertyModifierDefinition::Readonly { .. }
            )
        })
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                PromotedPropertyModifierDefinition::Protected { .. } => Some(Visibility::Protected),
                PromotedPropertyModifierDefinition::Private { .. } => Some(Visibility::Private),
                PromotedPropertyModifierDefinition::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}

impl PropertyModifierDefinitionGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn get_readonly(&self) -> Option<&PropertyModifierDefinition> {
        self.modifiers
            .iter()
            .find(|modifier| matches!(modifier, PropertyModifierDefinition::Readonly { .. }))
    }

    pub fn get_static(&self) -> Option<&PropertyModifierDefinition> {
        self.modifiers
            .iter()
            .find(|modifier| matches!(modifier, PropertyModifierDefinition::Static { .. }))
    }

    pub fn has_readonly(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, PropertyModifierDefinition::Readonly { .. }))
    }

    pub fn has_static(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, PropertyModifierDefinition::Static { .. }))
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                PropertyModifierDefinition::Protected { .. } => Some(Visibility::Protected),
                PropertyModifierDefinition::Private { .. } => Some(Visibility::Private),
                PropertyModifierDefinition::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}

impl MethodModifierDefinitionGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn has_final(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, MethodModifierDefinition::Final { .. }))
    }

    pub fn has_static(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, MethodModifierDefinition::Static { .. }))
    }

    pub fn has_abstract(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, MethodModifierDefinition::Abstract { .. }))
    }

    pub fn get_abstract(&self) -> Option<&MethodModifierDefinition> {
        self.modifiers
            .iter()
            .find(|modifier| matches!(modifier, MethodModifierDefinition::Abstract { .. }))
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                MethodModifierDefinition::Protected { .. } => Some(Visibility::Protected),
                MethodModifierDefinition::Private { .. } => Some(Visibility::Private),
                MethodModifierDefinition::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}

impl ClassModifierDefinitionGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn has_final(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ClassModifierDefinition::Final { .. }))
    }

    pub fn has_readonly(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ClassModifierDefinition::Readonly { .. }))
    }

    pub fn has_abstract(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ClassModifierDefinition::Abstract { .. }))
    }
}

impl ConstantModifierDefinitionGroup {
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    pub fn has_final(&self) -> bool {
        self.modifiers
            .iter()
            .any(|modifier| matches!(modifier, ConstantModifierDefinition::Final { .. }))
    }

    pub fn visibility(&self) -> Visibility {
        self.modifiers
            .iter()
            .find_map(|modifier| match modifier {
                ConstantModifierDefinition::Protected { .. } => Some(Visibility::Protected),
                ConstantModifierDefinition::Private { .. } => Some(Visibility::Private),
                ConstantModifierDefinition::Public { .. } => Some(Visibility::Public),
                _ => None,
            })
            .unwrap_or(Visibility::Public)
    }
}

impl Node for VisibilityModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            VisibilityModifierDefinition::Public(span) => span.position,
            VisibilityModifierDefinition::Protected(span) => span.position,
            VisibilityModifierDefinition::Private(span) => span.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            VisibilityModifierDefinition::Public(span) => span.position + 6,
            VisibilityModifierDefinition::Protected(span) => span.position + 9,
            VisibilityModifierDefinition::Private(span) => span.position + 7,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for PromotedPropertyModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            PromotedPropertyModifierDefinition::Public(span) => span.position,
            PromotedPropertyModifierDefinition::Protected(span) => span.position,
            PromotedPropertyModifierDefinition::Private(span) => span.position,
            PromotedPropertyModifierDefinition::Readonly(span) => span.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            PromotedPropertyModifierDefinition::Public(span) => span.position + 6,
            PromotedPropertyModifierDefinition::Protected(span) => span.position + 9,
            PromotedPropertyModifierDefinition::Private(span) => span.position + 7,
            PromotedPropertyModifierDefinition::Readonly(span) => span.position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for PropertyModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            PropertyModifierDefinition::Static(span) => span.position,
            PropertyModifierDefinition::Public(span) => span.position,
            PropertyModifierDefinition::Protected(span) => span.position,
            PropertyModifierDefinition::Private(span) => span.position,
            PropertyModifierDefinition::Readonly(span) => span.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            PropertyModifierDefinition::Static(span) => span.position + 6,
            PropertyModifierDefinition::Public(span) => span.position + 6,
            PropertyModifierDefinition::Protected(span) => span.position + 9,
            PropertyModifierDefinition::Private(span) => span.position + 7,
            PropertyModifierDefinition::Readonly(span) => span.position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for MethodModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            MethodModifierDefinition::Static(span) => span.position,
            MethodModifierDefinition::Public(span) => span.position,
            MethodModifierDefinition::Protected(span) => span.position,
            MethodModifierDefinition::Private(span) => span.position,
            MethodModifierDefinition::Final(span) => span.position,
            MethodModifierDefinition::Abstract(span) => span.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            MethodModifierDefinition::Static(span) => span.position + 6,
            MethodModifierDefinition::Public(span) => span.position + 6,
            MethodModifierDefinition::Protected(span) => span.position + 9,
            MethodModifierDefinition::Private(span) => span.position + 7,
            MethodModifierDefinition::Final(span) => span.position + 5,
            MethodModifierDefinition::Abstract(span) => span.position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for ClassModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            ClassModifierDefinition::Final(span) => span.position,
            ClassModifierDefinition::Abstract(span) => span.position,
            ClassModifierDefinition::Readonly(span) => span.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ClassModifierDefinition::Final(span) => span.position + 5,
            ClassModifierDefinition::Abstract(span) => span.position + 8,
            ClassModifierDefinition::Readonly(span) => span.position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for ConstantModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            ConstantModifierDefinition::Final(span) => span.position,
            ConstantModifierDefinition::Public(span) => span.position,
            ConstantModifierDefinition::Protected(span) => span.position,
            ConstantModifierDefinition::Private(span) => span.position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ConstantModifierDefinition::Final(span) => span.position + 5,
            ConstantModifierDefinition::Public(span) => span.position + 6,
            ConstantModifierDefinition::Protected(span) => span.position + 9,
            ConstantModifierDefinition::Private(span) => span.position + 7,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl std::fmt::Display for PromotedPropertyModifierDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromotedPropertyModifierDefinition::Public(_) => write!(f, "public"),
            PromotedPropertyModifierDefinition::Protected(_) => write!(f, "protected"),
            PromotedPropertyModifierDefinition::Private(_) => write!(f, "private"),
            PromotedPropertyModifierDefinition::Readonly(_) => write!(f, "readonly"),
        }
    }
}
