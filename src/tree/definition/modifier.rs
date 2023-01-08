use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::Node;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum VisibilityModifierDefinition {
    Public(usize),
    Protected(usize),
    Private(usize),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum PromotedPropertyModifierDefinition {
    Public(usize),
    Protected(usize),
    Private(usize),
    Readonly(usize),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct PromotedPropertyModifierDefinitionGroup {
    pub modifiers: Vec<PromotedPropertyModifierDefinition>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum PropertyModifierDefinition {
    Public(usize),
    Protected(usize),
    Private(usize),
    Static(usize),
    Readonly(usize),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct PropertyModifierDefinitionGroup {
    pub modifiers: Vec<PropertyModifierDefinition>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum MethodModifierDefinition {
    Final(usize),
    Static(usize),
    Abstract(usize),
    Public(usize),
    Protected(usize),
    Private(usize),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct MethodModifierDefinitionGroup {
    pub modifiers: Vec<MethodModifierDefinition>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ClassModifierDefinition {
    Final(usize),
    Abstract(usize),
    Readonly(usize),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct ClassModifierDefinitionGroup {
    pub modifiers: Vec<ClassModifierDefinition>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ConstantModifierDefinition {
    Final(usize),
    Public(usize),
    Protected(usize),
    Private(usize),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(transparent)]
pub struct ConstantModifierDefinitionGroup {
    pub modifiers: Vec<ConstantModifierDefinition>,
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
            VisibilityModifierDefinition::Public(position) => *position,
            VisibilityModifierDefinition::Protected(position) => *position,
            VisibilityModifierDefinition::Private(position) => *position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            VisibilityModifierDefinition::Public(position) => position + 6,
            VisibilityModifierDefinition::Protected(position) => position + 9,
            VisibilityModifierDefinition::Private(position) => position + 7,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for PromotedPropertyModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            PromotedPropertyModifierDefinition::Public(position) => *position,
            PromotedPropertyModifierDefinition::Protected(position) => *position,
            PromotedPropertyModifierDefinition::Private(position) => *position,
            PromotedPropertyModifierDefinition::Readonly(position) => *position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            PromotedPropertyModifierDefinition::Public(position) => position + 6,
            PromotedPropertyModifierDefinition::Protected(position) => position + 9,
            PromotedPropertyModifierDefinition::Private(position) => position + 7,
            PromotedPropertyModifierDefinition::Readonly(position) => position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for PropertyModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            PropertyModifierDefinition::Static(position) => *position,
            PropertyModifierDefinition::Public(position) => *position,
            PropertyModifierDefinition::Protected(position) => *position,
            PropertyModifierDefinition::Private(position) => *position,
            PropertyModifierDefinition::Readonly(position) => *position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            PropertyModifierDefinition::Static(position) => position + 6,
            PropertyModifierDefinition::Public(position) => position + 6,
            PropertyModifierDefinition::Protected(position) => position + 9,
            PropertyModifierDefinition::Private(position) => position + 7,
            PropertyModifierDefinition::Readonly(position) => position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for MethodModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            MethodModifierDefinition::Static(position) => *position,
            MethodModifierDefinition::Public(position) => *position,
            MethodModifierDefinition::Protected(position) => *position,
            MethodModifierDefinition::Private(position) => *position,
            MethodModifierDefinition::Final(position) => *position,
            MethodModifierDefinition::Abstract(position) => *position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            MethodModifierDefinition::Static(position) => position + 6,
            MethodModifierDefinition::Public(position) => position + 6,
            MethodModifierDefinition::Protected(position) => position + 9,
            MethodModifierDefinition::Private(position) => position + 7,
            MethodModifierDefinition::Final(position) => *position + 5,
            MethodModifierDefinition::Abstract(position) => position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for ClassModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            ClassModifierDefinition::Final(position) => *position,
            ClassModifierDefinition::Abstract(position) => *position,
            ClassModifierDefinition::Readonly(position) => *position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ClassModifierDefinition::Final(position) => *position + 5,
            ClassModifierDefinition::Abstract(position) => position + 8,
            ClassModifierDefinition::Readonly(position) => position + 8,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}

impl Node for ConstantModifierDefinition {
    fn initial_position(&self) -> usize {
        match self {
            ConstantModifierDefinition::Final(position) => *position,
            ConstantModifierDefinition::Public(position) => *position,
            ConstantModifierDefinition::Protected(position) => *position,
            ConstantModifierDefinition::Private(position) => *position,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            ConstantModifierDefinition::Final(position) => position + 5,
            ConstantModifierDefinition::Public(position) => position + 6,
            ConstantModifierDefinition::Protected(position) => position + 9,
            ConstantModifierDefinition::Private(position) => position + 7,
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
