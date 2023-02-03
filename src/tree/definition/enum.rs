use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::constant::ClassishConstantDefinition;
use crate::tree::definition::function::MethodDefinition;
use crate::tree::expression::Expression;
use crate::tree::identifier::Identifier;
use crate::tree::identifier::TemplatedIdentifier;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum EnumDefinition {
    Backed(BackedEnumDefinition),
    Unit(UnitEnumDefinition),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnitEnumDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#enum: Keyword,
    pub name: Identifier,
    pub implements: Option<EnumImplementsDefinition>,
    pub body: UnitEnumBodyDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EnumImplementsDefinition {
    pub implements: Keyword,
    pub interfaces: CommaSeparated<TemplatedIdentifier>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnitEnumBodyDefinition {
    pub left_brace: usize,
    pub members: Vec<UnitEnumMemberDefinition>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum UnitEnumMemberDefinition {
    Case(UnitEnumCaseDefinition),
    Method(MethodDefinition),
    Constant(ClassishConstantDefinition),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnitEnumCaseDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    pub case: Keyword,
    pub name: Identifier,
    pub semicolon: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BackedEnumDefinition {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#enum: Keyword,
    pub name: Identifier,
    pub backed_type: BackedEnumTypeDefinition,
    pub implements: Option<EnumImplementsDefinition>,
    pub body: BackedEnumBodyDefinition,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum BackedEnumTypeDefinition {
    String(usize, Identifier),
    Int(usize, Identifier),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BackedEnumBodyDefinition {
    pub left_brace: usize,
    pub members: Vec<BackedEnumMemberDefinition>,
    pub right_brace: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum BackedEnumMemberDefinition {
    Case(BackedEnumCaseDefinition),
    Method(MethodDefinition),
    Constant(ClassishConstantDefinition),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BackedEnumCaseDefinition {
    pub attributes: Vec<AttributeGroupDefinition>,
    pub case: Keyword,
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
        match &self {
            Self::Backed(definition) => definition.initial_position(),
            Self::Unit(definition) => definition.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::Backed(definition) => definition.final_position(),
            Self::Unit(definition) => definition.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::Backed(definition) => vec![definition],
            Self::Unit(definition) => vec![definition],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::Backed(definition) => definition.get_description(),
            Self::Unit(definition) => definition.get_description(),
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
            self.r#enum.initial_position()
        }
    }

    fn final_position(&self) -> usize {
        self.body.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#enum, &self.name];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(implements) = &self.implements {
            children.push(implements);
        }

        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "unit enum definition".to_string()
    }
}

impl Node for EnumImplementsDefinition {
    fn initial_position(&self) -> usize {
        self.implements.initial_position()
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

        self.implements.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.implements];

        for interface in &self.interfaces.inner {
            children.push(interface);
        }

        children
    }

    fn get_description(&self) -> String {
        "enum implements definition".to_string()
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

    fn get_description(&self) -> String {
        "unit enum body definition".to_string()
    }
}

impl Node for UnitEnumMemberDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Case(case) => case.initial_position(),
            Self::Method(method) => method.initial_position(),
            Self::Constant(constant) => constant.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
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

    fn get_description(&self) -> String {
        match &self {
            Self::Case(case) => case.get_description(),
            Self::Method(method) => method.get_description(),
            Self::Constant(constant) => constant.get_description(),
        }
    }
}

impl Node for UnitEnumCaseDefinition {
    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            attributes.initial_position()
        } else {
            self.case.initial_position()
        }
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.case];
        for attribute in &self.attributes {
            children.push(attribute);
        }

        children
    }

    fn get_description(&self) -> String {
        "unit enum case definition".to_string()
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
            self.r#enum.initial_position()
        }
    }

    fn final_position(&self) -> usize {
        self.body.right_brace + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#enum, &self.name, &self.backed_type];
        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(implements) = &self.implements {
            children.push(implements);
        }

        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "backed enum definition".to_string()
    }
}

impl Node for BackedEnumTypeDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::String(colon, _) | Self::Int(colon, _) => *colon,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::String(_, identifier) | Self::Int(_, identifier) => identifier.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::String(_, identifier) | Self::Int(_, identifier) => vec![identifier],
        }
    }

    fn get_description(&self) -> String {
        "backed enum type definition".to_string()
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

    fn get_description(&self) -> String {
        "backed enum body definition".to_string()
    }
}

impl Node for BackedEnumMemberDefinition {
    fn initial_position(&self) -> usize {
        match &self {
            Self::Case(case) => case.initial_position(),
            Self::Method(method) => method.initial_position(),
            Self::Constant(constant) => constant.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
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

    fn get_description(&self) -> String {
        match &self {
            Self::Case(case) => case.get_description(),
            Self::Method(method) => method.get_description(),
            Self::Constant(constant) => constant.get_description(),
        }
    }
}

impl Node for BackedEnumCaseDefinition {
    fn initial_position(&self) -> usize {
        if let Some(attributes) = self.attributes.first() {
            attributes.initial_position()
        } else {
            self.case.initial_position()
        }
    }

    fn final_position(&self) -> usize {
        self.semicolon + 1
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.case, &self.name, &self.value]
    }

    fn get_description(&self) -> String {
        "backed enum case definition".to_string()
    }
}
