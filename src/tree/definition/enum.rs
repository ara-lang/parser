use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::function::ConcreteMethodDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum EnumDefinition {
    Backed(BackedEnumDefinition),
    Unit(UnitEnumDefinition),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnitEnumDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#enum: usize,
    pub name: Identifier,
    pub implements: Option<EnumImplementsDefinition>,
    pub body: UnitEnumBodyDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EnumImplementsDefinition {
    pub implements: usize,
    pub interfaces: CommaSeparated<TemplatedIdentifier>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnitEnumBodyDefinition {
    pub left_brace: usize,
    pub members: Vec<UnitEnumMemberDefinition>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum UnitEnumMemberDefinition {
    Case(UnitEnumCaseDefinition),
    Method(ConcreteMethodDefinition),
    Constant(ClassishConstantDefinition),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnitEnumCaseDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    pub start: usize,
    pub name: Identifier,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BackedEnumDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#enum: usize,
    pub name: Identifier,
    pub backed_type: BackedEnumTypeDefinition,
    pub implements: Option<EnumImplementsDefinition>,
    pub body: BackedEnumBodyDefinition,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum BackedEnumTypeDefinition {
    String(usize, Identifier),
    Int(usize, Identifier),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BackedEnumBodyDefinition {
    pub left_brace: usize,
    pub members: Vec<BackedEnumMemberDefinition>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum BackedEnumMemberDefinition {
    Case(BackedEnumCaseDefinition),
    Method(ConcreteMethodDefinition),
    Constant(ClassishConstantDefinition),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BackedEnumCaseDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    pub case: usize,
    pub name: Identifier,
    pub equals: usize,
    pub value: Expression,
    pub semicolon: usize,
}

impl Node for EnumDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match self {
            EnumDefinition::Backed(definition) => definition.initial_position(),
            EnumDefinition::Unit(definition) => definition.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            EnumDefinition::Backed(definition) => definition.final_position(),
            EnumDefinition::Unit(definition) => definition.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            EnumDefinition::Backed(definition) => vec![definition],
            EnumDefinition::Unit(definition) => vec![definition],
        }
    }
}

impl Node for UnitEnumDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            attributes.initial_position()
        } else {
            self.r#enum
        }
    }

    fn final_position(&self) -> usize {
        self.body.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.name];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(implements) = &self.implements {
            children.push(implements);
        }

        children.push(&self.body);

        children
    }
}

impl Node for EnumImplementsDefinition {
    fn initial_position(&self) -> usize {
        self.implements
    }

    fn final_position(&self) -> usize {
        if let Some(last_interface) = self.interfaces.inner.last() {
            let last_interface_position = last_interface.final_position();
            if let Some(last_comma) = self.interfaces.commas.last() {
                let last_comma_position = last_comma + 1;
                if last_comma_position > last_interface_position {
                    return last_comma_position;
                }
            }

            return last_interface_position;
        }

        self.implements + 10
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.interfaces
            .inner
            .iter()
            .map(|interface| interface as &dyn Node)
            .collect()
    }
}

impl Node for UnitEnumBodyDefinition {
    fn initial_position(&self) -> usize {
        self.left_brace
    }

    fn final_position(&self) -> usize {
        self.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .iter()
            .map(|member| member as &dyn Node)
            .collect()
    }
}

impl Node for UnitEnumMemberDefinition {
    fn initial_position(&self) -> usize {
        match self {
            Self::Case(case) => case.initial_position(),
            Self::Method(method) => method.initial_position(),
            Self::Constant(constant) => constant.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            Self::Case(case) => case.final_position(),
            Self::Method(method) => method.final_position(),
            Self::Constant(constant) => constant.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Self::Case(case) => vec![case],
            Self::Method(method) => vec![method],
            Self::Constant(constant) => vec![constant],
        }
    }
}

impl Node for UnitEnumCaseDefinition {
    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            attributes.initial_position()
        } else {
            self.start
        }
    }

    fn final_position(&self) -> usize {
        self.end + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];
        for attribute in &self.attributes {
            children.push(attribute);
        }

        children
    }
}

impl Node for BackedEnumDefinition {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            attributes.initial_position()
        } else {
            self.r#enum
        }
    }

    fn final_position(&self) -> usize {
        self.body.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.name, &self.backed_type];
        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(implements) = &self.implements {
            children.push(implements);
        }

        children.push(&self.body);

        children
    }
}

impl Node for BackedEnumTypeDefinition {
    fn initial_position(&self) -> usize {
        match self {
            Self::String(start, _) => *start,
            Self::Int(start, _) => *start,
        }
    }

    fn final_position(&self) -> usize {
        match self {
            Self::String(_, identifier) => identifier.final_position(),
            Self::Int(_, identifier) => identifier.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match self {
            Self::String(_, identifier) => vec![identifier],
            Self::Int(_, identifier) => vec![identifier],
        }
    }
}

impl Node for BackedEnumBodyDefinition {
    fn initial_position(&self) -> usize {
        self.left_brace
    }

    fn final_position(&self) -> usize {
        self.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        self.members
            .iter()
            .map(|member| member as &dyn Node)
            .collect()
    }
}

impl Node for BackedEnumMemberDefinition {
    fn initial_position(&self) -> usize {
        match self {
            Self::Case(case) => case.initial_position(),
            Self::Method(method) => method.initial_position(),
            Self::Constant(constant) => constant.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match self {
            Self::Case(case) => case.final_position(),
            Self::Method(method) => method.final_position(),
            Self::Constant(constant) => constant.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Case(case) => vec![case],
            Self::Method(method) => vec![method],
            Self::Constant(constant) => vec![constant],
        }
    }
}

impl Node for BackedEnumCaseDefinition {
    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            attributes.initial_position()
        } else {
            self.case
        }
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.name, &self.value]
    }
}
